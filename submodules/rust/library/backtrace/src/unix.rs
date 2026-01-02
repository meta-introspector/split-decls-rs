mkuse!{use split_decls_genesis :: ourprelude :: * ;}
mkuse!{#[doc = " Backtrace support built on libgcc with some extra OS-specific support"] #[doc = ""] #[doc = " Some methods of getting a backtrace:"] #[doc = ""] #[doc = " * The backtrace() functions on unix. It turns out this doesn't work very"] #[doc = "   well for green threads on OSX, and the address to symbol portion of it"] #[doc = "   suffers problems that are described below."] #[doc = ""] #[doc = " * Using libunwind. This is more difficult than it sounds because libunwind"] #[doc = "   isn't installed everywhere by default. It's also a bit of a hefty library,"] #[doc = "   so possibly not the best option. When testing, libunwind was excellent at"] #[doc = "   getting both accurate backtraces and accurate symbols across platforms."] #[doc = "   This route was not chosen in favor of the next option, however."] #[doc = ""] #[doc = " * We're already using libgcc_s for exceptions in rust (triggering thread"] #[doc = "   unwinding and running destructors on the stack), and it turns out that it"] #[doc = "   conveniently comes with a function that also gives us a backtrace. All of"] #[doc = "   these functions look like _Unwind_*, but it's not quite the full"] #[doc = "   repertoire of the libunwind API. Due to it already being in use, this was"] #[doc = "   the chosen route of getting a backtrace."] #[doc = ""] #[doc = " After choosing libgcc_s for backtraces, the sad part is that it will only"] #[doc = " give us a stack trace of instruction pointers. Thankfully these instruction"] #[doc = " pointers are accurate (they work for green and native threads), but it's"] #[doc = " then up to us again to figure out how to translate these addresses to"] #[doc = " symbols. As with before, we have a few options. Before, that, a little bit"] #[doc = " of an interlude about symbols. This is my very limited knowledge about"] #[doc = " symbol tables, and this information is likely slightly wrong, but the"] #[doc = " general idea should be correct."] #[doc = ""] #[doc = " When talking about symbols, it's helpful to know a few things about where"] #[doc = " symbols are located. Some symbols are located in the dynamic symbol table"] #[doc = " of the executable which in theory means that they're available for dynamic"] #[doc = " linking and lookup. Other symbols end up only in the local symbol table of"] #[doc = " the file. This loosely corresponds to pub and priv functions in Rust."] #[doc = ""] #[doc = " Armed with this knowledge, we know that our solution for address to symbol"] #[doc = " translation will need to consult both the local and dynamic symbol tables."] #[doc = " With that in mind, here's our options of translating an address to"] #[doc = " a symbol."] #[doc = ""] #[doc = " * Use dladdr(). The original backtrace()-based idea actually uses dladdr()"] #[doc = "   behind the scenes to translate, and this is why backtrace() was not used."] #[doc = "   Conveniently, this method works fantastically on OSX. It appears dladdr()"] #[doc = "   uses magic to consult the local symbol table, or we're putting everything"] #[doc = "   in the dynamic symbol table anyway. Regardless, for OSX, this is the"] #[doc = "   method used for translation. It's provided by the system and easy to do."] #[doc = ""] #[doc = "   Sadly, all other systems have a dladdr() implementation that does not"] #[doc = "   consult the local symbol table. This means that most functions are blank"] #[doc = "   because they don't have symbols. This means that we need another solution."] #[doc = ""] #[doc = " * Use unw_get_proc_name(). This is part of the libunwind api (not the"] #[doc = "   libgcc_s version of the libunwind api), but involves taking a dependency"] #[doc = "   to libunwind. We may pursue this route in the future if we bundle"] #[doc = "   libunwind, but libunwind was unwieldy enough that it was not chosen at"] #[doc = "   this time to provide this functionality."] #[doc = ""] #[doc = " * Shell out to a utility like `readelf`. Crazy though it may sound, it's a"] #[doc = "   semi-reasonable solution. The stdlib already knows how to spawn processes,"] #[doc = "   so in theory it could invoke readelf, parse the output, and consult the"] #[doc = "   local/dynamic symbol tables from there. This ended up not getting chosen"] #[doc = "   due to the craziness of the idea plus the advent of the next option."] #[doc = ""] #[doc = " * Use `libbacktrace`. It turns out that this is a small library bundled in"] #[doc = "   the gcc repository which provides backtrace and symbol translation"] #[doc = "   functionality. All we really need from it is the backtrace functionality,"] #[doc = "   and we only really need this on everything that's not OSX, so this is the"] #[doc = "   chosen route for now."] #[doc = ""] #[doc = " In summary, the current situation uses libgcc_s to get a trace of stack"] #[doc = " pointers, and we use dladdr() or libbacktrace to translate these addresses"] #[doc = " to symbols. This is a bit of a hokey implementation as-is, but it works for"] #[doc = " all unix platforms we support right now, so it at least gets the job done."] use prelude :: v1 :: * ;}
mkuse!{use io :: prelude :: * ;}
mkuse!{use ffi :: CStr ;}
mkuse!{use io ;}
mkuse!{use libc ;}
mkuse!{use mem ;}
mkuse!{use str ;}
mkuse!{use sync :: StaticMutex ;}
mkuse!{use sys_common :: backtrace :: * ;}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    #[cfg (any (target_os = "macos" , target_os = "ios"))] fn print (w : & mut Write , idx : isize , addr : * mut libc :: c_void , _symaddr : * mut libc :: c_void) -> io :: Result < () > { use intrinsics ; #[repr (C)] struct Dl_info { dli_fname : * const libc :: c_char , dli_fbase : * mut libc :: c_void , dli_sname : * const libc :: c_char , dli_saddr : * mut libc :: c_void , } extern { fn dladdr (addr : * const libc :: c_void , info : * mut Dl_info) -> libc :: c_int ; } let mut info : Dl_info = unsafe { intrinsics :: init () } ; if unsafe { dladdr (addr , & mut info) == 0 } { output (w , idx , addr , None) } else { output (w , idx , addr , Some (unsafe { CStr :: from_ptr (info . dli_sname) . to_bytes () })) } }
}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    #[cfg (not (any (target_os = "macos" , target_os = "ios")))] fn print (w : & mut Write , idx : isize , addr : * mut libc :: c_void , symaddr : * mut libc :: c_void) -> io :: Result < () > { use env ; use os :: unix :: prelude :: * ; use ptr ; type backtrace_syminfo_callback = extern "C" fn (data : * mut libc :: c_void , pc : libc :: uintptr_t , symname : * const libc :: c_char , symval : libc :: uintptr_t , symsize : libc :: uintptr_t) ; type backtrace_full_callback = extern "C" fn (data : * mut libc :: c_void , pc : libc :: uintptr_t , filename : * const libc :: c_char , lineno : libc :: c_int , function : * const libc :: c_char) -> libc :: c_int ; type backtrace_error_callback = extern "C" fn (data : * mut libc :: c_void , msg : * const libc :: c_char , errnum : libc :: c_int) ; enum backtrace_state { } #[link (name = "backtrace" , kind = "static")] #[cfg (not (test))] extern { } extern { fn backtrace_create_state (filename : * const libc :: c_char , threaded : libc :: c_int , error : backtrace_error_callback , data : * mut libc :: c_void) -> * mut backtrace_state ; fn backtrace_syminfo (state : * mut backtrace_state , addr : libc :: uintptr_t , cb : backtrace_syminfo_callback , error : backtrace_error_callback , data : * mut libc :: c_void) -> libc :: c_int ; fn backtrace_pcinfo (state : * mut backtrace_state , addr : libc :: uintptr_t , cb : backtrace_full_callback , error : backtrace_error_callback , data : * mut libc :: c_void) -> libc :: c_int ; } type FileLine = (* const libc :: c_char , libc :: c_int) ; extern fn error_cb (_data : * mut libc :: c_void , _msg : * const libc :: c_char , _errnum : libc :: c_int) { } extern fn syminfo_cb (data : * mut libc :: c_void , _pc : libc :: uintptr_t , symname : * const libc :: c_char , _symval : libc :: uintptr_t , _symsize : libc :: uintptr_t) { let slot = data as * mut * const libc :: c_char ; unsafe { * slot = symname ; } } extern fn pcinfo_cb (data : * mut libc :: c_void , _pc : libc :: uintptr_t , filename : * const libc :: c_char , lineno : libc :: c_int , _function : * const libc :: c_char) -> libc :: c_int { if ! filename . is_null () { let slot = data as * mut & mut [FileLine] ; let buffer = unsafe { ptr :: read (slot) } ; if ! buffer . is_empty () { buffer [0] = (filename , lineno) ; unsafe { ptr :: write (slot , & mut buffer [1 ..]) ; } } } 0 } unsafe fn init_state () -> * mut backtrace_state { static mut STATE : * mut backtrace_state = 0 as * mut backtrace_state ; static mut LAST_FILENAME : [libc :: c_char ; 256] = [0 ; 256] ; if ! STATE . is_null () { return STATE } let selfname = if cfg ! (target_os = "freebsd") || cfg ! (target_os = "dragonfly") || cfg ! (target_os = "bitrig") || cfg ! (target_os = "openbsd") { env :: current_exe () . ok () } else { None } ; let filename = match selfname { Some (path) => { let bytes = path . as_os_str () . as_bytes () ; if bytes . len () < LAST_FILENAME . len () { let i = bytes . iter () ; for (slot , val) in LAST_FILENAME . iter_mut () . zip (i) { * slot = * val as libc :: c_char ; } LAST_FILENAME . as_ptr () } else { ptr :: null () } } None => ptr :: null () , } ; STATE = backtrace_create_state (filename , 0 , error_cb , ptr :: null_mut ()) ; return STATE } let state = unsafe { init_state () } ; if state . is_null () { return output (w , idx , addr , None) } let mut data = ptr :: null () ; let data_addr = & mut data as * mut * const libc :: c_char ; let ret = unsafe { backtrace_syminfo (state , symaddr as libc :: uintptr_t , syminfo_cb , error_cb , data_addr as * mut libc :: c_void) } ; if ret == 0 || data . is_null () { try ! (output (w , idx , addr , None)) ; } else { try ! (output (w , idx , addr , Some (unsafe { CStr :: from_ptr (data) . to_bytes () }))) ; } const FILELINE_SIZE : usize = 32 ; let mut fileline_buf = [(ptr :: null () , - 1) ; FILELINE_SIZE] ; let ret ; let fileline_count ; { let mut fileline_win : & mut [FileLine] = & mut fileline_buf ; let fileline_addr = & mut fileline_win as * mut & mut [FileLine] ; ret = unsafe { backtrace_pcinfo (state , addr as libc :: uintptr_t , pcinfo_cb , error_cb , fileline_addr as * mut libc :: c_void) } ; fileline_count = FILELINE_SIZE - fileline_win . len () ; } if ret == 0 { for (i , & (file , line)) in fileline_buf [.. fileline_count] . iter () . enumerate () { if file . is_null () { continue ; } let file = unsafe { CStr :: from_ptr (file) . to_bytes () } ; try ! (output_fileline (w , file , line , i == FILELINE_SIZE - 1)) ; } } Ok (()) }
}

macro_rules! output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output in module {}", module_path!());
    };
}

mkfn!{
    output_introspect!();
    fn output (w : & mut Write , idx : isize , addr : * mut libc :: c_void , s : Option < & [u8] >) -> io :: Result < () > { try ! (write ! (w , "  {:2}: {:2$?} - " , idx , addr , HEX_WIDTH)) ; match s . and_then (| s | str :: from_utf8 (s) . ok ()) { Some (string) => try ! (demangle (w , string)) , None => try ! (write ! (w , "<unknown>")) , } w . write_all (& ['\n' as u8]) }
}

macro_rules! output_fileline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output_fileline in module {}", module_path!());
    };
}

mkfn!{
    output_fileline_introspect!();
    #[allow (dead_code)] fn output_fileline (w : & mut Write , file : & [u8] , line : libc :: c_int , more : bool) -> io :: Result < () > { let file = str :: from_utf8 (file) . unwrap_or ("<unknown>") ; try ! (write ! (w , "      {:3$}at {}:{}" , "" , file , line , HEX_WIDTH)) ; if more { try ! (write ! (w , " <... and possibly more>")) ; } w . write_all (& ['\n' as u8]) }
}
mkmod!{uw, { 
                getname!(uw);
                getsrc!(uw);
                getpath!(uw);
                get_deps!(uw);
                get_crates!(uw);
                mkinclude!(uw);
                mkuse!{pub use self :: _Unwind_Reason_Code :: * ;}
mkuse!{use libc ;}
mkitem!{mkenum!{#[repr (C)] pub enum _Unwind_Reason_Code { _URC_NO_REASON = 0 , _URC_FOREIGN_EXCEPTION_CAUGHT = 1 , _URC_FATAL_PHASE2_ERROR = 2 , _URC_FATAL_PHASE1_ERROR = 3 , _URC_NORMAL_STOP = 4 , _URC_END_OF_STACK = 5 , _URC_HANDLER_FOUND = 6 , _URC_INSTALL_CONTEXT = 7 , _URC_CONTINUE_UNWIND = 8 , _URC_FAILURE = 9 , }}}
mkitem!{mkenum!{pub enum _Unwind_Context { }}}
mkitem!{pub type _Unwind_Trace_Fn = extern fn (ctx : * mut _Unwind_Context , arg : * mut libc :: c_void) -> _Unwind_Reason_Code ;}
mkitem!{extern { #[cfg (not (all (target_os = "ios" , target_arch = "arm")))] pub fn _Unwind_Backtrace (trace : _Unwind_Trace_Fn , trace_argument : * mut libc :: c_void) -> _Unwind_Reason_Code ; #[cfg (all (not (all (target_os = "android" , target_arch = "arm")) , not (all (target_os = "linux" , target_arch = "arm"))))] pub fn _Unwind_GetIPInfo (ctx : * mut _Unwind_Context , ip_before_insn : * mut libc :: c_int) -> libc :: uintptr_t ; #[cfg (all (not (target_os = "android") , not (all (target_os = "linux" , target_arch = "arm"))))] pub fn _Unwind_FindEnclosingFunction (pc : * mut libc :: c_void) -> * mut libc :: c_void ; }}

macro_rules! _Unwind_GetIP_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetIP in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetIP_introspect!();
    #[cfg (any (all (target_os = "android" , target_arch = "arm") , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_GetIP (ctx : * mut _Unwind_Context) -> libc :: uintptr_t { #[repr (C)] enum _Unwind_VRS_Result { _UVRSR_OK = 0 , _UVRSR_NOT_IMPLEMENTED = 1 , _UVRSR_FAILED = 2 , } #[repr (C)] enum _Unwind_VRS_RegClass { _UVRSC_CORE = 0 , _UVRSC_VFP = 1 , _UVRSC_FPA = 2 , _UVRSC_WMMXD = 3 , _UVRSC_WMMXC = 4 , } #[repr (C)] enum _Unwind_VRS_DataRepresentation { _UVRSD_UINT32 = 0 , _UVRSD_VFPX = 1 , _UVRSD_FPAX = 2 , _UVRSD_UINT64 = 3 , _UVRSD_FLOAT = 4 , _UVRSD_DOUBLE = 5 , } type _Unwind_Word = libc :: c_uint ; extern { fn _Unwind_VRS_Get (ctx : * mut _Unwind_Context , klass : _Unwind_VRS_RegClass , word : _Unwind_Word , repr : _Unwind_VRS_DataRepresentation , data : * mut libc :: c_void) -> _Unwind_VRS_Result ; } let mut val : _Unwind_Word = 0 ; let ptr = & mut val as * mut _Unwind_Word ; let _ = _Unwind_VRS_Get (ctx , _Unwind_VRS_RegClass :: _UVRSC_CORE , 15 , _Unwind_VRS_DataRepresentation :: _UVRSD_UINT32 , ptr as * mut libc :: c_void) ; (val & ! 1) as libc :: uintptr_t }
}

macro_rules! _Unwind_GetIPInfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetIPInfo in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetIPInfo_introspect!();
    #[cfg (any (all (target_os = "android" , target_arch = "arm") , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_GetIPInfo (ctx : * mut _Unwind_Context , ip_before_insn : * mut libc :: c_int) -> libc :: uintptr_t { * ip_before_insn = 0 ; _Unwind_GetIP (ctx) }
}

macro_rules! _Unwind_FindEnclosingFunction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_FindEnclosingFunction in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_FindEnclosingFunction_introspect!();
    #[cfg (any (target_os = "android" , all (target_os = "linux" , target_arch = "arm")))] pub unsafe fn _Unwind_FindEnclosingFunction (pc : * mut libc :: c_void) -> * mut libc :: c_void { pc }
} 
            }}