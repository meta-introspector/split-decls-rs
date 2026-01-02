mkuse!{use super :: super :: { dbghelp , windows_sys :: * } ;}
mkuse!{use super :: { BytesOrWideString , ResolveWhat , SymbolName } ;}
mkuse!{use core :: cmp ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: marker ;}
mkuse!{use core :: mem :: { self , MaybeUninit } ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: slice ;}

macro_rules! ptr_from_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_from_ref in module {}", module_path!());
    };
}

mkfn!{
    ptr_from_ref_introspect!();
    # [inline (always)] # [must_use] const fn ptr_from_ref < T : ? Sized > (r : & T) -> * const T { r }
}
mkitem!{mkstruct!{pub struct Symbol < 'a > { name : * const [u8] , addr : * mut c_void , line : Option < u32 > , filename : Option < * const [u16] > , # [cfg (feature = "std")] _filename_cache : Option < :: std :: ffi :: OsString > , # [cfg (not (feature = "std"))] _filename_cache : () , _marker : marker :: PhantomData < & 'a i32 > , }}}
mkitem!{mkimpl!{impl Symbol < '_ > { pub fn name (& self) -> Option < SymbolName < '_ > > { Some (SymbolName :: new (unsafe { & * self . name })) } pub fn addr (& self) -> Option < * mut c_void > { Some (self . addr) } pub fn filename_raw (& self) -> Option < BytesOrWideString < '_ > > { self . filename . map (| slice | unsafe { BytesOrWideString :: Wide (& * slice) }) } pub fn colno (& self) -> Option < u32 > { None } pub fn lineno (& self) -> Option < u32 > { self . line } # [cfg (feature = "std")] pub fn filename (& self) -> Option < & :: std :: path :: Path > { use std :: path :: Path ; self . _filename_cache . as_ref () . map (Path :: new) } }}}
mkitem!{mkstruct!{# [repr (C , align (8))] struct Aligned8 < T > (T) ;}}

macro_rules! resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve in module {}", module_path!());
    };
}

mkfn!{
    resolve_introspect!();
    # [cfg (not (target_vendor = "win7"))] pub unsafe fn resolve (what : ResolveWhat < '_ > , cb : & mut dyn FnMut (& super :: Symbol)) { let dbghelp = match dbghelp :: init () { Ok (dbghelp) => dbghelp , Err (()) => return , } ; unsafe { match what { ResolveWhat :: Address (_) => { resolve_with_inline (& dbghelp , what . address_or_ip () , None , cb) } ResolveWhat :: Frame (frame) => { resolve_with_inline (& dbghelp , frame . ip () , frame . inner . inline_context () , cb) } } ; } }
}

macro_rules! resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve in module {}", module_path!());
    };
}

mkfn!{
    resolve_introspect!();
    # [cfg (target_vendor = "win7")] pub unsafe fn resolve (what : ResolveWhat < '_ > , cb : & mut dyn FnMut (& super :: Symbol)) { let dbghelp = match dbghelp :: init () { Ok (dbghelp) => dbghelp , Err (()) => return , } ; unsafe { let resolve_inner = if (* dbghelp . dbghelp ()) . SymAddrIncludeInlineTrace () . is_some () { resolve_with_inline } else { resolve_legacy } ; match what { ResolveWhat :: Address (_) => resolve_inner (& dbghelp , what . address_or_ip () , None , cb) , ResolveWhat :: Frame (frame) => { resolve_inner (& dbghelp , frame . ip () , frame . inner . inline_context () , cb) } } ; } }
}

macro_rules! resolve_legacy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_legacy in module {}", module_path!());
    };
}

mkfn!{
    resolve_legacy_introspect!();
    # [doc = " Resolve the address using the legacy dbghelp API."] # [doc = ""] # [doc = " This should work all the way down to Windows XP. The inline context is"] # [doc = " ignored, since this concept was only introduced in dbghelp 6.2+."] # [cfg (target_vendor = "win7")] unsafe fn resolve_legacy (dbghelp : & dbghelp :: Init , addr : * mut c_void , _inline_context : Option < u32 > , cb : & mut dyn FnMut (& super :: Symbol) ,) -> Option < () > { let addr = super :: adjust_ip (addr) as u64 ; unsafe { do_resolve (| info | dbghelp . SymFromAddrW () (GetCurrentProcess () , addr , & mut 0 , info) , | line | dbghelp . SymGetLineFromAddrW64 () (GetCurrentProcess () , addr , & mut 0 , line) , cb ,) ; } Some (()) }
}

macro_rules! resolve_with_inline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_with_inline in module {}", module_path!());
    };
}

mkfn!{
    resolve_with_inline_introspect!();
    # [doc = " Resolve the address using the modern dbghelp APIs."] # [doc = ""] # [doc = " Note that calling this function requires having dbghelp 6.2+ loaded - and"] # [doc = " will panic otherwise."] unsafe fn resolve_with_inline (dbghelp : & dbghelp :: Init , addr : * mut c_void , inline_context : Option < u32 > , cb : & mut dyn FnMut (& super :: Symbol) ,) -> Option < () > { unsafe { let current_process = GetCurrentProcess () ; let SymFromInlineContextW = (* dbghelp . dbghelp ()) . SymFromInlineContextW () ? ; let SymGetLineFromInlineContextW = (* dbghelp . dbghelp ()) . SymGetLineFromInlineContextW () ? ; let addr = super :: adjust_ip (addr) as u64 ; let (inlined_frame_count , inline_context) = if let Some (ic) = inline_context { (0 , ic) } else { let SymAddrIncludeInlineTrace = (* dbghelp . dbghelp ()) . SymAddrIncludeInlineTrace () ? ; let SymQueryInlineTrace = (* dbghelp . dbghelp ()) . SymQueryInlineTrace () ? ; let mut inlined_frame_count = SymAddrIncludeInlineTrace (current_process , addr) ; let mut inline_context = 0 ; if (inlined_frame_count > 0 && SymQueryInlineTrace (current_process , addr , 0 , addr , addr , & mut inline_context , & mut 0 ,) != TRUE) || inlined_frame_count == 0 { inlined_frame_count = 0 ; inline_context = 0 ; } (inlined_frame_count , inline_context) } ; let last_inline_context = inline_context + 1 + inlined_frame_count ; for inline_context in inline_context .. last_inline_context { do_resolve (| info | SymFromInlineContextW (current_process , addr , inline_context , & mut 0 , info) , | line | { SymGetLineFromInlineContextW (current_process , addr , inline_context , 0 , & mut 0 , line ,) } , cb ,) ; } } Some (()) }
}

macro_rules! do_resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function do_resolve in module {}", module_path!());
    };
}

mkfn!{
    do_resolve_introspect!();
    # [doc = " This function is only meant to be called with certain Windows API functions as its arguments,"] # [doc = " using closures to simplify away here-unspecified arguments:"] # [doc = " - `sym_from_addr`: either `SymFromAddrW` or `SymFromInlineContextW`"] # [doc = " - `get_line_from_addr`: `SymGetLineFromAddrW64` or `SymGetLineFromInlineContextW`"] unsafe fn do_resolve (sym_from_addr : impl FnOnce (* mut SYMBOL_INFOW) -> BOOL , get_line_from_addr : impl FnOnce (& mut IMAGEHLP_LINEW64) -> BOOL , cb : & mut dyn FnMut (& super :: Symbol) ,) { const SIZE : usize = 2 * MAX_SYM_NAME as usize + mem :: size_of :: < SYMBOL_INFOW > () ; let mut data = MaybeUninit :: < Aligned8 < [u8 ; SIZE] > > :: zeroed () ; let info = data . as_mut_ptr () . cast :: < SYMBOL_INFOW > () ; unsafe { (* info) . MaxNameLen = MAX_SYM_NAME as u32 } ; unsafe { (* info) . SizeOfStruct = 88 } ; if sym_from_addr (info) != TRUE { return ; } let name_len = unsafe { cmp :: min ((* info) . NameLen as usize , (* info) . MaxNameLen as usize - 1) } ; let name_ptr = unsafe { (& raw const (* info) . Name) . cast :: < u16 > () } ; let mut name_buffer = [0_u8 ; 256] ; let mut name_len = unsafe { WideCharToMultiByte (CP_UTF8 , 0 , name_ptr , name_len as i32 , name_buffer . as_mut_ptr () , name_buffer . len () as i32 , core :: ptr :: null_mut () , core :: ptr :: null_mut () ,) as usize } ; if name_len == 0 { name_len = name_buffer . len () ; } else if name_len > name_buffer . len () { return ; } let name = ptr :: addr_of ! (name_buffer [.. name_len]) ; let mut line = IMAGEHLP_LINEW64 { SizeOfStruct : 0 , Key : core :: ptr :: null_mut () , LineNumber : 0 , FileName : core :: ptr :: null_mut () , Address : 0 , } ; line . SizeOfStruct = mem :: size_of :: < IMAGEHLP_LINEW64 > () as u32 ; let mut filename = None ; let mut lineno = None ; if get_line_from_addr (& mut line) == TRUE { lineno = Some (line . LineNumber) ; let base = line . FileName ; let mut len = 0 ; while unsafe { * base . offset (len) != 0 } { len += 1 ; } let len = len as usize ; unsafe { filename = Some (ptr_from_ref (slice :: from_raw_parts (base , len))) ; } } cb (& super :: Symbol { inner : Symbol { name , addr : unsafe { (* info) . Address } as * mut _ , line : lineno , filename , _filename_cache : unsafe { cache (filename) } , _marker : marker :: PhantomData , } , }) }
}

macro_rules! cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cache in module {}", module_path!());
    };
}

mkfn!{
    cache_introspect!();
    # [cfg (feature = "std")] unsafe fn cache (filename : Option < * const [u16] >) -> Option < :: std :: ffi :: OsString > { use std :: os :: windows :: ffi :: OsStringExt ; unsafe { filename . map (| f | :: std :: ffi :: OsString :: from_wide (& * f)) } }
}

macro_rules! cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cache in module {}", module_path!());
    };
}

mkfn!{
    cache_introspect!();
    # [cfg (not (feature = "std"))] unsafe fn cache (_filename : Option < * const [u16] >) { }
}

macro_rules! clear_symbol_cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clear_symbol_cache in module {}", module_path!());
    };
}

mkfn!{
    clear_symbol_cache_introspect!();
    pub unsafe fn clear_symbol_cache () { }
}