mkuse!{use super :: { MIN_ALIGN , realloc_fallback } ;}
mkuse!{use crate :: alloc :: { GlobalAlloc , Layout , System } ;}
mkuse!{use crate :: ffi :: c_void ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sys :: c ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{const HEAP_ZERO_MEMORY : u32 = 0x00000008 ;}
mkitem!{windows_targets :: link ! ("kernel32.dll" "system" fn GetProcessHeap () -> c :: HANDLE) ;}
mkitem!{windows_targets :: link ! ("kernel32.dll" "system" fn HeapAlloc (hheap : c :: HANDLE , dwflags : u32 , dwbytes : usize) -> * mut c_void) ;}
mkitem!{windows_targets :: link ! ("kernel32.dll" "system" fn HeapReAlloc (hheap : c :: HANDLE , dwflags : u32 , lpmem : * const c_void , dwbytes : usize) -> * mut c_void) ;}
mkitem!{windows_targets :: link ! ("kernel32.dll" "system" fn HeapFree (hheap : c :: HANDLE , dwflags : u32 , lpmem : * const c_void) -> c :: BOOL) ;}

macro_rules! get_process_heap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_process_heap in module {}", module_path!());
    };
}

mkfn!{
    get_process_heap_introspect!();
    fn get_process_heap () -> * mut c_void { unsafe { GetProcessHeap () } }
}

macro_rules! process_heap_alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function process_heap_alloc in module {}", module_path!());
    };
}

mkfn!{
    process_heap_alloc_introspect!();
    # [inline (never)] fn process_heap_alloc (_heap : MaybeUninit < c :: HANDLE > , flags : u32 , bytes : usize ,) -> * mut c_void { let heap = get_process_heap () ; if core :: intrinsics :: unlikely (heap . is_null ()) { return ptr :: null_mut () ; } unsafe { HeapAlloc (heap , flags , bytes) } }
}
mkitem!{mkstruct!{# [repr (C)] struct Header (* mut u8) ;}}

macro_rules! allocate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function allocate in module {}", module_path!());
    };
}

mkfn!{
    allocate_introspect!();
    # [inline] unsafe fn allocate (layout : Layout , zeroed : bool) -> * mut u8 { let flags = if zeroed { HEAP_ZERO_MEMORY } else { 0 } ; if layout . align () <= MIN_ALIGN { process_heap_alloc (MaybeUninit :: uninit () , flags , layout . size ()) as * mut u8 } else { let total = layout . align () + layout . size () ; let ptr = process_heap_alloc (MaybeUninit :: uninit () , flags , total) as * mut u8 ; if ptr . is_null () { return ptr :: null_mut () ; } let offset = layout . align () - (ptr . addr () & (layout . align () - 1)) ; let aligned = unsafe { ptr . add (offset) } ; unsafe { ptr :: write ((aligned as * mut Header) . sub (1) , Header (ptr)) } ; aligned } }
}
mkitem!{mkimpl!{# [stable (feature = "alloc_system_type" , since = "1.28.0")] unsafe impl GlobalAlloc for System { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let zeroed = false ; unsafe { allocate (layout , zeroed) } } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { let zeroed = true ; unsafe { allocate (layout , zeroed) } } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { let block = { if layout . align () <= MIN_ALIGN { ptr } else { unsafe { ptr :: read ((ptr as * mut Header) . sub (1)) . 0 } } } ; let heap = get_process_heap () ; unsafe { HeapFree (heap , 0 , block . cast :: < c_void > ()) } ; } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { if layout . align () <= MIN_ALIGN { let heap = get_process_heap () ; unsafe { HeapReAlloc (heap , 0 , ptr . cast :: < c_void > () , new_size) . cast :: < u8 > () } } else { unsafe { realloc_fallback (self , ptr , layout , new_size) } } } }}}