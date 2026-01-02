mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use super :: windows_sys :: * ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: mem ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: slice ;}
mkitem!{macro_rules ! dbghelp { (extern "system" { $ (fn $ name : ident ($ ($ arg : ident : $ argty : ty) ,*) -> $ ret : ty ;) * }) => (pub struct Dbghelp { # [doc = " The loaded DLL for `dbghelp.dll`"] dll : HINSTANCE , $ ($ name : usize ,) * } static mut DBGHELP : Dbghelp = Dbghelp { dll : ptr :: null_mut () , $ ($ name : 0 ,) * } ; $ (pub type $ name = unsafe extern "system" fn ($ ($ argty) ,*) -> $ ret ;) * impl Dbghelp { # [doc = " Attempts to open `dbghelp.dll`. Returns success if it works or"] # [doc = " error if `LoadLibraryW` fails."] fn ensure_open (& mut self) -> Result < () , () > { if ! self . dll . is_null () { return Ok (()) } let lib = b"dbghelp.dll\0" ; unsafe { self . dll = LoadLibraryA (lib . as_ptr ()) ; if self . dll . is_null () { Err (()) } else { Ok (()) } } } $ (pub fn $ name (& mut self) -> Option <$ name > { cfg_if :: cfg_if ! { if # [cfg (any (target_arch = "x86" , not (windows_raw_dylib)))] { let _ : unsafe extern "system" fn ($ ($ argty) ,*) -> $ ret = super :: windows_sys ::$ name ; } else { let _ : unsafe extern "C" fn ($ ($ argty) ,*) -> $ ret = super :: windows_sys ::$ name ; } } unsafe { if self .$ name == 0 { let name = concat ! (stringify ! ($ name) , "\0") ; self .$ name = self . symbol (name . as_bytes ()) ?; } Some (mem :: transmute ::< usize , $ name > (self .$ name)) } }) * fn symbol (& self , symbol : & [u8]) -> Option < usize > { unsafe { GetProcAddress (self . dll , symbol . as_ptr ()) . map (| address | address as usize) } } } # [allow (dead_code)] impl Init { $ (pub fn $ name (& self) -> $ name { # [allow (static_mut_refs)] unsafe { DBGHELP .$ name () . unwrap () } }) * pub fn dbghelp (& self) -> * mut Dbghelp { # [allow (unused_unsafe)] unsafe { ptr :: addr_of_mut ! (DBGHELP) } } }) }}
mkitem!{dbghelp ! { extern "system" { fn SymGetOptions () -> u32 ; fn SymSetOptions (options : u32) -> u32 ; fn SymInitializeW (handle : HANDLE , path : PCWSTR , invade : BOOL) -> BOOL ; fn SymGetSearchPathW (hprocess : HANDLE , searchpatha : PWSTR , searchpathlength : u32) -> BOOL ; fn SymSetSearchPathW (hprocess : HANDLE , searchpatha : PCWSTR) -> BOOL ; fn EnumerateLoadedModulesW64 (hprocess : HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : * const c_void) -> BOOL ; fn StackWalk64 (MachineType : u32 , hProcess : HANDLE , hThread : HANDLE , StackFrame : * mut STACKFRAME64 , ContextRecord : * mut c_void , ReadMemoryRoutine : PREAD_PROCESS_MEMORY_ROUTINE64 , FunctionTableAccessRoutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , GetModuleBaseRoutine : PGET_MODULE_BASE_ROUTINE64 , TranslateAddress : PTRANSLATE_ADDRESS_ROUTINE64) -> BOOL ; fn SymFunctionTableAccess64 (hProcess : HANDLE , AddrBase : u64) -> * mut c_void ; fn SymGetModuleBase64 (hProcess : HANDLE , AddrBase : u64) -> u64 ; fn SymFromAddrW (hProcess : HANDLE , Address : u64 , Displacement : * mut u64 , Symbol : * mut SYMBOL_INFOW) -> BOOL ; fn SymGetLineFromAddrW64 (hProcess : HANDLE , dwAddr : u64 , pdwDisplacement : * mut u32 , Line : * mut IMAGEHLP_LINEW64) -> BOOL ; fn StackWalkEx (MachineType : u32 , hProcess : HANDLE , hThread : HANDLE , StackFrame : * mut STACKFRAME_EX , ContextRecord : * mut c_void , ReadMemoryRoutine : PREAD_PROCESS_MEMORY_ROUTINE64 , FunctionTableAccessRoutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , GetModuleBaseRoutine : PGET_MODULE_BASE_ROUTINE64 , TranslateAddress : PTRANSLATE_ADDRESS_ROUTINE64 , Flags : u32) -> BOOL ; fn SymFromInlineContextW (hProcess : HANDLE , Address : u64 , InlineContext : u32 , Displacement : * mut u64 , Symbol : * mut SYMBOL_INFOW) -> BOOL ; fn SymGetLineFromInlineContextW (hProcess : HANDLE , dwAddr : u64 , InlineContext : u32 , qwModuleBaseAddress : u64 , pdwDisplacement : * mut u32 , Line : * mut IMAGEHLP_LINEW64) -> BOOL ; fn SymAddrIncludeInlineTrace (hProcess : HANDLE , Address : u64) -> u32 ; fn SymQueryInlineTrace (hProcess : HANDLE , StartAddress : u64 , StartContext : u32 , StartRetAddress : u64 , CurAddress : u64 , CurContext : * mut u32 , CurFrameIndex : * mut u32) -> BOOL ; } }}
mkitem!{mkstruct!{pub struct Init { lock : HANDLE , }}}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [doc = " Initialize all support necessary to access `dbghelp` API functions from this"] # [doc = " crate."] # [doc = ""] # [doc = " Note that this function is **safe**, it internally has its own"] # [doc = " synchronization. Also note that it is safe to call this function multiple"] # [doc = " times recursively."] pub fn init () -> Result < Init , () > { use core :: sync :: atomic :: { AtomicPtr , Ordering :: SeqCst } ; fn mutex_name () -> [u8 ; 33] { let mut name : [u8 ; 33] = * b"Local\\RustBacktraceMutex00000000\0" ; let mut id = unsafe { GetCurrentProcessId () } ; let mut index = name . len () - 1 ; while id > 0 { name [index - 1] = match (id & 0xF) as u8 { h @ 0 ..= 9 => b'0' + h , h => b'A' + (h - 10) , } ; id >>= 4 ; index -= 1 ; } name } unsafe { static LOCK : AtomicPtr < c_void > = AtomicPtr :: new (ptr :: null_mut ()) ; let mut lock = LOCK . load (SeqCst) ; if lock . is_null () { let name = mutex_name () ; lock = CreateMutexA (ptr :: null_mut () , FALSE , name . as_ptr ()) ; if lock . is_null () { return Err (()) ; } if let Err (other) = LOCK . compare_exchange (ptr :: null_mut () , lock , SeqCst , SeqCst) { debug_assert ! (! other . is_null ()) ; CloseHandle (lock) ; lock = other ; } } debug_assert ! (! lock . is_null ()) ; let r = WaitForSingleObjectEx (lock , INFINITE , FALSE) ; debug_assert_eq ! (r , 0) ; let ret = Init { lock } ; # [allow (static_mut_refs)] DBGHELP . ensure_open () ? ; static mut INITIALIZED : bool = false ; if ! INITIALIZED { set_optional_options (ret . dbghelp ()) ; INITIALIZED = true ; } Ok (ret) } }
}

macro_rules! set_optional_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_optional_options in module {}", module_path!());
    };
}

mkfn!{
    set_optional_options_introspect!();
    unsafe fn set_optional_options (dbghelp : * mut Dbghelp) -> Option < () > { unsafe { let orig = (* dbghelp) . SymGetOptions () ? () ; (* dbghelp) . SymSetOptions () ? (orig | SYMOPT_DEFERRED_LOADS) ; (* dbghelp) . SymInitializeW () ? (GetCurrentProcess () , ptr :: null_mut () , TRUE) ; let mut search_path_buf = Vec :: new () ; search_path_buf . resize (1024 , 0) ; if (* dbghelp) . SymGetSearchPathW () ? (GetCurrentProcess () , search_path_buf . as_mut_ptr () , search_path_buf . len () as _ ,) == TRUE { let len = lstrlenW (search_path_buf . as_mut_ptr ()) ; assert ! (len >= 0) ; search_path_buf . truncate (len as usize) ; } else { search_path_buf . clear () ; search_path_buf . push (utf16_char ('.')) ; search_path_buf . push (utf16_char (';')) ; } let mut search_path = SearchPath :: new (search_path_buf) ; (* dbghelp) . EnumerateLoadedModulesW64 () ? (GetCurrentProcess () , Some (enum_loaded_modules_callback) , ((& mut search_path) as * mut SearchPath) as * mut c_void ,) ; let new_search_path = search_path . finalize () ; (* dbghelp) . SymSetSearchPathW () ? (GetCurrentProcess () , new_search_path . as_ptr ()) ; } Some (()) }
}
mkitem!{mkstruct!{struct SearchPath { search_path_utf16 : Vec < u16 > , }}}

macro_rules! utf16_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf16_char in module {}", module_path!());
    };
}

mkfn!{
    utf16_char_introspect!();
    fn utf16_char (c : char) -> u16 { let buf = & mut [0u16 ; 2] ; let buf = c . encode_utf16 (buf) ; assert ! (buf . len () == 1) ; buf [0] }
}
mkitem!{mkimpl!{impl SearchPath { fn new (initial_search_path : Vec < u16 >) -> Self { Self { search_path_utf16 : initial_search_path , } } # [doc = " Add a path to the search path if it is not already present."] fn add (& mut self , path : & [u16]) { let sep = utf16_char (';') ; if ! self . search_path_utf16 . split (| & c | c == sep) . any (| p | p == path) { if self . search_path_utf16 . last () != Some (& sep) { self . search_path_utf16 . push (sep) ; } self . search_path_utf16 . extend_from_slice (path) ; } } fn finalize (mut self) -> Vec < u16 > { self . search_path_utf16 . push (0) ; self . search_path_utf16 } }}}

macro_rules! enum_loaded_modules_callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enum_loaded_modules_callback in module {}", module_path!());
    };
}

mkfn!{
    enum_loaded_modules_callback_introspect!();
    extern "system" fn enum_loaded_modules_callback (module_name : PCWSTR , _ : u64 , _ : u32 , user_context : * const c_void ,) -> BOOL { let len : usize = unsafe { lstrlenW (module_name) . try_into () . unwrap () } ; if len == 0 { return TRUE ; } let module_name = unsafe { slice :: from_raw_parts (module_name , len) } ; let path_sep = utf16_char ('\\') ; let alt_path_sep = utf16_char ('/') ; let Some (end_of_directory) = module_name . iter () . rposition (| & c | c == path_sep || c == alt_path_sep) else { return TRUE ; } ; let search_path = unsafe { & mut * (user_context as * mut SearchPath) } ; search_path . add (& module_name [.. end_of_directory]) ; TRUE }
}
mkitem!{mkimpl!{impl Drop for Init { fn drop (& mut self) { unsafe { let r = ReleaseMutex (self . lock) ; debug_assert ! (r != 0) ; } } }}}