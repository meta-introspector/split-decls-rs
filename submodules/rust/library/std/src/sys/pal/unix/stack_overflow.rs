mkuse!{pub use self :: imp :: { cleanup , init } ;}
mkuse!{use self :: imp :: { drop_handler , make_handler } ;}
mkitem!{mkstruct!{pub struct Handler { data : * mut libc :: c_void , }}}
mkitem!{mkimpl!{impl Handler { pub unsafe fn new (thread_name : Option < Box < str > >) -> Handler { make_handler (false , thread_name) } fn null () -> Handler { Handler { data : crate :: ptr :: null_mut () } } }}}
mkitem!{mkimpl!{impl Drop for Handler { fn drop (& mut self) { unsafe { drop_handler (self . data) ; } } }}}
mkmod!{thread_info, { 
                getname!(thread_info);
                getsrc!(thread_info);
                getpath!(thread_info);
                get_deps!(thread_info);
                get_crates!(thread_info);
                mkinclude!(thread_info);
                 
            }}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkuse!{use libc :: { MAP_ANON , MAP_FAILED , MAP_FIXED , MAP_PRIVATE , PROT_NONE , PROT_READ , PROT_WRITE , SA_ONSTACK , SA_SIGINFO , SIG_DFL , SIGBUS , SIGSEGV , SS_DISABLE , sigaction , sigaltstack , sighandler_t , } ;}
mkuse!{# [cfg (not (all (target_os = "linux" , target_env = "gnu")))] use libc :: { mmap as mmap64 , mprotect , munmap } ;}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use libc :: { mmap64 , mprotect , munmap } ;}
mkuse!{use super :: Handler ;}
mkuse!{use super :: thread_info :: { delete_current_info , set_current_info , with_current_info } ;}
mkuse!{use crate :: ops :: Range ;}
mkuse!{use crate :: sync :: OnceLock ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicPtr , AtomicUsize , Ordering } ;}
mkuse!{use crate :: sys :: pal :: unix :: os ;}
mkuse!{use crate :: { io , mem , panic , ptr } ;}

macro_rules! signal_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function signal_handler in module {}", module_path!());
    };
}

mkfn!{
    signal_handler_introspect!();
    # [doc = " SIGSEGV/SIGBUS entry point"] # [doc = " # Safety"] # [doc = " Rust doesn't call this, it *gets called*."] # [forbid (unsafe_op_in_unsafe_fn)] unsafe extern "C" fn signal_handler (signum : libc :: c_int , info : * mut libc :: siginfo_t , _data : * mut libc :: c_void ,) { let fault_addr = unsafe { (* info) . si_addr () . addr () } ; if fault_addr != 0 { with_current_info (| thread_info | { if let Some (thread_info) = thread_info && thread_info . guard_page_range . contains (& fault_addr) { let name = thread_info . thread_name . as_deref () . unwrap_or ("<unknown>") ; let tid = crate :: thread :: current_os_id () ; rtprintpanic ! ("\nthread '{name}' ({tid}) has overflowed its stack\n") ; rtabort ! ("stack overflow") ; } }) } let mut action : sigaction = unsafe { mem :: zeroed () } ; action . sa_sigaction = SIG_DFL ; unsafe { sigaction (signum , & action , ptr :: null_mut ()) } ; }
}
mkitem!{static PAGE_SIZE : Atomic < usize > = AtomicUsize :: new (0) ;}
mkitem!{static MAIN_ALTSTACK : Atomic < * mut libc :: c_void > = AtomicPtr :: new (ptr :: null_mut ()) ;}
mkitem!{static NEED_ALTSTACK : Atomic < bool > = AtomicBool :: new (false) ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [doc = " # Safety"] # [doc = " Must be called only once"] # [forbid (unsafe_op_in_unsafe_fn)] pub unsafe fn init () { PAGE_SIZE . store (os :: page_size () , Ordering :: Relaxed) ; let mut guard_page_range = unsafe { install_main_guard () } ; let mut action : sigaction = unsafe { mem :: zeroed () } ; for & signal in & [SIGSEGV , SIGBUS] { unsafe { sigaction (signal , ptr :: null_mut () , & mut action) } ; if action . sa_sigaction == SIG_DFL { if ! NEED_ALTSTACK . load (Ordering :: Relaxed) { NEED_ALTSTACK . store (true , Ordering :: Release) ; let handler = unsafe { make_handler (true , None) } ; MAIN_ALTSTACK . store (handler . data , Ordering :: Relaxed) ; mem :: forget (handler) ; if let Some (guard_page_range) = guard_page_range . take () { set_current_info (guard_page_range , Some (Box :: from ("main"))) ; } } action . sa_flags = SA_SIGINFO | SA_ONSTACK ; action . sa_sigaction = signal_handler as sighandler_t ; unsafe { sigaction (signal , & action , ptr :: null_mut ()) } ; } } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    # [doc = " # Safety"] # [doc = " Must be called only once"] # [forbid (unsafe_op_in_unsafe_fn)] pub unsafe fn cleanup () { unsafe { drop_handler (MAIN_ALTSTACK . load (Ordering :: Relaxed)) } ; }
}

macro_rules! get_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_stack in module {}", module_path!());
    };
}

mkfn!{
    get_stack_introspect!();
    unsafe fn get_stack () -> libc :: stack_t { # [cfg (any (target_os = "openbsd" , target_os = "netbsd" , target_os = "linux" , target_os = "dragonfly" ,))] let flags = MAP_PRIVATE | MAP_ANON | libc :: MAP_STACK ; # [cfg (not (any (target_os = "openbsd" , target_os = "netbsd" , target_os = "linux" , target_os = "dragonfly" ,)))] let flags = MAP_PRIVATE | MAP_ANON ; let sigstack_size = sigstack_size () ; let page_size = PAGE_SIZE . load (Ordering :: Relaxed) ; let stackp = mmap64 (ptr :: null_mut () , sigstack_size + page_size , PROT_READ | PROT_WRITE , flags , - 1 , 0 ,) ; if stackp == MAP_FAILED { panic ! ("failed to allocate an alternative stack: {}" , io :: Error :: last_os_error ()) ; } let guard_result = libc :: mprotect (stackp , page_size , PROT_NONE) ; if guard_result != 0 { panic ! ("failed to set up alternative stack guard page: {}" , io :: Error :: last_os_error ()) ; } let stackp = stackp . add (page_size) ; libc :: stack_t { ss_sp : stackp , ss_flags : 0 , ss_size : sigstack_size } }
}

macro_rules! make_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_handler in module {}", module_path!());
    };
}

mkfn!{
    make_handler_introspect!();
    # [doc = " # Safety"] # [doc = " Mutates the alternate signal stack"] # [forbid (unsafe_op_in_unsafe_fn)] pub unsafe fn make_handler (main_thread : bool , thread_name : Option < Box < str > >) -> Handler { if ! NEED_ALTSTACK . load (Ordering :: Acquire) { return Handler :: null () ; } if ! main_thread { if let Some (guard_page_range) = unsafe { current_guard () } { set_current_info (guard_page_range , thread_name) ; } } let mut stack = unsafe { mem :: zeroed () } ; unsafe { sigaltstack (ptr :: null () , & mut stack) } ; if stack . ss_flags & SS_DISABLE != 0 { unsafe { stack = get_stack () ; sigaltstack (& stack , ptr :: null_mut ()) ; } Handler { data : stack . ss_sp as * mut libc :: c_void } } else { Handler :: null () } }
}

macro_rules! drop_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_handler in module {}", module_path!());
    };
}

mkfn!{
    drop_handler_introspect!();
    # [doc = " # Safety"] # [doc = " Must be called"] # [doc = " - only with our handler or nullptr"] # [doc = " - only when done with our altstack"] # [doc = " This disables the alternate signal stack!"] # [forbid (unsafe_op_in_unsafe_fn)] pub unsafe fn drop_handler (data : * mut libc :: c_void) { if ! data . is_null () { let sigstack_size = sigstack_size () ; let page_size = PAGE_SIZE . load (Ordering :: Relaxed) ; let disabling_stack = libc :: stack_t { ss_sp : ptr :: null_mut () , ss_flags : SS_DISABLE , ss_size : sigstack_size , } ; unsafe { sigaltstack (& disabling_stack , ptr :: null_mut ()) } ; unsafe { munmap (data . sub (page_size) , sigstack_size + page_size) } ; } delete_current_info () ; }
}

macro_rules! sigstack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sigstack_size in module {}", module_path!());
    };
}

mkfn!{
    sigstack_size_introspect!();
    # [doc = " Modern kernels on modern hardware can have dynamic signal stack sizes."] # [cfg (any (target_os = "linux" , target_os = "android"))] fn sigstack_size () -> usize { let dynamic_sigstksz = unsafe { libc :: getauxval (libc :: AT_MINSIGSTKSZ) } ; libc :: SIGSTKSZ . max (dynamic_sigstksz as _) }
}

macro_rules! sigstack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sigstack_size in module {}", module_path!());
    };
}

mkfn!{
    sigstack_size_introspect!();
    # [doc = " Not all OS support hardware where this is needed."] # [cfg (not (any (target_os = "linux" , target_os = "android")))] fn sigstack_size () -> usize { libc :: SIGSTKSZ }
}

macro_rules! get_stack_start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_stack_start in module {}", module_path!());
    };
}

mkfn!{
    get_stack_start_introspect!();
    # [cfg (any (target_os = "solaris" , target_os = "illumos"))] unsafe fn get_stack_start () -> Option < * mut libc :: c_void > { let mut current_stack : libc :: stack_t = crate :: mem :: zeroed () ; assert_eq ! (libc :: stack_getbounds (& mut current_stack) , 0) ; Some (current_stack . ss_sp) }
}

macro_rules! get_stack_start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_stack_start in module {}", module_path!());
    };
}

mkfn!{
    get_stack_start_introspect!();
    # [cfg (target_os = "macos")] unsafe fn get_stack_start () -> Option < * mut libc :: c_void > { let th = libc :: pthread_self () ; let stackptr = libc :: pthread_get_stackaddr_np (th) ; Some (stackptr . map_addr (| addr | addr - libc :: pthread_get_stacksize_np (th))) }
}

macro_rules! get_stack_start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_stack_start in module {}", module_path!());
    };
}

mkfn!{
    get_stack_start_introspect!();
    # [cfg (target_os = "openbsd")] unsafe fn get_stack_start () -> Option < * mut libc :: c_void > { let mut current_stack : libc :: stack_t = crate :: mem :: zeroed () ; assert_eq ! (libc :: pthread_stackseg_np (libc :: pthread_self () , & mut current_stack) , 0) ; let stack_ptr = current_stack . ss_sp ; let stackaddr = if libc :: pthread_main_np () == 1 { stack_ptr . addr () - current_stack . ss_size + PAGE_SIZE . load (Ordering :: Relaxed) } else { stack_ptr . addr () - current_stack . ss_size } ; Some (stack_ptr . with_addr (stackaddr)) }
}

macro_rules! get_stack_start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_stack_start in module {}", module_path!());
    };
}

mkfn!{
    get_stack_start_introspect!();
    # [cfg (any (target_os = "android" , target_os = "freebsd" , target_os = "netbsd" , target_os = "hurd" , target_os = "linux" , target_os = "l4re"))] unsafe fn get_stack_start () -> Option < * mut libc :: c_void > { let mut ret = None ; let mut attr : mem :: MaybeUninit < libc :: pthread_attr_t > = mem :: MaybeUninit :: uninit () ; if ! cfg ! (target_os = "freebsd") { attr = mem :: MaybeUninit :: zeroed () ; } # [cfg (target_os = "freebsd")] assert_eq ! (libc :: pthread_attr_init (attr . as_mut_ptr ()) , 0) ; # [cfg (target_os = "freebsd")] let e = libc :: pthread_attr_get_np (libc :: pthread_self () , attr . as_mut_ptr ()) ; # [cfg (not (target_os = "freebsd"))] let e = libc :: pthread_getattr_np (libc :: pthread_self () , attr . as_mut_ptr ()) ; if e == 0 { let mut stackaddr = crate :: ptr :: null_mut () ; let mut stacksize = 0 ; assert_eq ! (libc :: pthread_attr_getstack (attr . as_ptr () , & mut stackaddr , & mut stacksize) , 0) ; ret = Some (stackaddr) ; } if e == 0 || cfg ! (target_os = "freebsd") { assert_eq ! (libc :: pthread_attr_destroy (attr . as_mut_ptr ()) , 0) ; } ret }
}

macro_rules! stack_start_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stack_start_aligned in module {}", module_path!());
    };
}

mkfn!{
    stack_start_aligned_introspect!();
    fn stack_start_aligned (page_size : usize) -> Option < * mut libc :: c_void > { let stackptr = unsafe { get_stack_start () ? } ; let stackaddr = stackptr . addr () ; let remainder = stackaddr % page_size ; Some (if remainder == 0 { stackptr } else { stackptr . with_addr (stackaddr + page_size - remainder) }) }
}

macro_rules! install_main_guard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard () -> Option < Range < usize > > { let page_size = PAGE_SIZE . load (Ordering :: Relaxed) ; unsafe { if cfg ! (all (target_os = "linux" , not (target_env = "musl"))) { install_main_guard_linux (page_size) } else if cfg ! (all (target_os = "linux" , target_env = "musl")) { install_main_guard_linux_musl (page_size) } else if cfg ! (target_os = "freebsd") { install_main_guard_freebsd (page_size) } else if cfg ! (any (target_os = "netbsd" , target_os = "openbsd")) { install_main_guard_bsds (page_size) } else { install_main_guard_default (page_size) } } }
}

macro_rules! install_main_guard_linux_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard_linux in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_linux_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard_linux (page_size : usize) -> Option < Range < usize > > { let stackptr = stack_start_aligned (page_size) ? ; let stackaddr = stackptr . addr () ; Some (stackaddr - page_size .. stackaddr) }
}

macro_rules! install_main_guard_linux_musl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard_linux_musl in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_linux_musl_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard_linux_musl (_page_size : usize) -> Option < Range < usize > > { None }
}

macro_rules! install_main_guard_freebsd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard_freebsd in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_freebsd_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard_freebsd (page_size : usize) -> Option < Range < usize > > { let stackptr = stack_start_aligned (page_size) ? ; let guardaddr = stackptr . addr () ; static PAGES : OnceLock < usize > = OnceLock :: new () ; let pages = PAGES . get_or_init (| | { use crate :: sys :: weak :: dlsym ; dlsym ! (fn sysctlbyname (name : * const libc :: c_char , oldp : * mut libc :: c_void , oldlenp : * mut libc :: size_t , newp : * const libc :: c_void , newlen : libc :: size_t ,) -> libc :: c_int ;) ; let mut guard : usize = 0 ; let mut size = size_of_val (& guard) ; let oid = c"security.bsd.stack_guard_page" ; match sysctlbyname . get () { Some (fcn) if unsafe { fcn (oid . as_ptr () , (& raw mut guard) . cast () , & raw mut size , ptr :: null_mut () , 0 ,) == 0 } => { guard } _ => 1 , } }) ; Some (guardaddr .. guardaddr + pages * page_size) }
}

macro_rules! install_main_guard_bsds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard_bsds in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_bsds_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard_bsds (page_size : usize) -> Option < Range < usize > > { let stackptr = stack_start_aligned (page_size) ? ; let stackaddr = stackptr . addr () ; Some (stackaddr - page_size .. stackaddr) }
}

macro_rules! install_main_guard_default_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install_main_guard_default in module {}", module_path!());
    };
}

mkfn!{
    install_main_guard_default_introspect!();
    # [forbid (unsafe_op_in_unsafe_fn)] unsafe fn install_main_guard_default (page_size : usize) -> Option < Range < usize > > { let stackptr = stack_start_aligned (page_size) ? ; let result = unsafe { mmap64 (stackptr , page_size , PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANON | MAP_FIXED , - 1 , 0 ,) } ; if result != stackptr || result == MAP_FAILED { panic ! ("failed to allocate a guard page: {}" , io :: Error :: last_os_error ()) ; } let result = unsafe { mprotect (stackptr , page_size , PROT_NONE) } ; if result != 0 { panic ! ("failed to protect the guard page: {}" , io :: Error :: last_os_error ()) ; } let guardaddr = stackptr . addr () ; Some (guardaddr .. guardaddr + page_size) }
}

macro_rules! current_guard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_guard in module {}", module_path!());
    };
}

mkfn!{
    current_guard_introspect!();
    # [cfg (any (target_os = "macos" , target_os = "openbsd" , target_os = "solaris" , target_os = "illumos" ,))] unsafe fn current_guard () -> Option < Range < usize > > { let stackptr = get_stack_start () ? ; let stackaddr = stackptr . addr () ; Some (stackaddr - PAGE_SIZE . load (Ordering :: Relaxed) .. stackaddr) }
}

macro_rules! current_guard_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_guard in module {}", module_path!());
    };
}

mkfn!{
    current_guard_introspect!();
    # [cfg (any (target_os = "android" , target_os = "freebsd" , target_os = "hurd" , target_os = "linux" , target_os = "netbsd" , target_os = "l4re"))] unsafe fn current_guard () -> Option < Range < usize > > { let mut ret = None ; let mut attr : mem :: MaybeUninit < libc :: pthread_attr_t > = mem :: MaybeUninit :: uninit () ; if ! cfg ! (target_os = "freebsd") { attr = mem :: MaybeUninit :: zeroed () ; } # [cfg (target_os = "freebsd")] assert_eq ! (libc :: pthread_attr_init (attr . as_mut_ptr ()) , 0) ; # [cfg (target_os = "freebsd")] let e = libc :: pthread_attr_get_np (libc :: pthread_self () , attr . as_mut_ptr ()) ; # [cfg (not (target_os = "freebsd"))] let e = libc :: pthread_getattr_np (libc :: pthread_self () , attr . as_mut_ptr ()) ; if e == 0 { let mut guardsize = 0 ; assert_eq ! (libc :: pthread_attr_getguardsize (attr . as_ptr () , & mut guardsize) , 0) ; if guardsize == 0 { if cfg ! (all (target_os = "linux" , target_env = "musl")) { guardsize = PAGE_SIZE . load (Ordering :: Relaxed) ; } else { panic ! ("there is no guard page") ; } } let mut stackptr = crate :: ptr :: null_mut :: < libc :: c_void > () ; let mut size = 0 ; assert_eq ! (libc :: pthread_attr_getstack (attr . as_ptr () , & mut stackptr , & mut size) , 0) ; let stackaddr = stackptr . addr () ; ret = if cfg ! (any (target_os = "freebsd" , target_os = "netbsd" , target_os = "hurd")) { Some (stackaddr - guardsize .. stackaddr) } else if cfg ! (all (target_os = "linux" , target_env = "musl")) { Some (stackaddr - guardsize .. stackaddr) } else if cfg ! (all (target_os = "linux" , any (target_env = "gnu" , target_env = "uclibc"))) { Some (stackaddr - guardsize .. stackaddr + guardsize) } else { Some (stackaddr .. stackaddr + guardsize) } ; } if e == 0 || cfg ! (target_os = "freebsd") { assert_eq ! (libc :: pthread_attr_destroy (attr . as_mut_ptr ()) , 0) ; } ret }
} 
            }}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                
macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub unsafe fn init () { }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { }
}

macro_rules! make_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_handler in module {}", module_path!());
    };
}

mkfn!{
    make_handler_introspect!();
    pub unsafe fn make_handler (_main_thread : bool , _thread_name : Option < Box < str > > ,) -> super :: Handler { super :: Handler :: null () }
}

macro_rules! drop_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_handler in module {}", module_path!());
    };
}

mkfn!{
    drop_handler_introspect!();
    pub unsafe fn drop_handler (_data : * mut libc :: c_void) { }
} 
            }}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkmod!{c, { 
                getname!(c);
                getsrc!(c);
                getpath!(c);
                get_deps!(c);
                get_crates!(c);
                mkinclude!(c);
                mkitem!{pub type PVECTORED_EXCEPTION_HANDLER = Option < unsafe extern "system" fn (exceptioninfo : * mut EXCEPTION_POINTERS) -> i32 > ;}
mkitem!{pub type NTSTATUS = i32 ;}
mkitem!{pub type BOOL = i32 ;}
mkitem!{unsafe extern "system" { pub fn AddVectoredExceptionHandler (first : u32 , handler : PVECTORED_EXCEPTION_HANDLER ,) -> * mut core :: ffi :: c_void ; pub fn SetThreadStackGuarantee (stacksizeinbytes : * mut u32) -> BOOL ; }}
mkitem!{pub const EXCEPTION_STACK_OVERFLOW : NTSTATUS = 0xC00000FD_u32 as _ ;}
mkitem!{pub const EXCEPTION_CONTINUE_SEARCH : i32 = 1i32 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone , Copy)] pub struct EXCEPTION_POINTERS { pub ExceptionRecord : * mut EXCEPTION_RECORD , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone , Copy)] pub struct EXCEPTION_RECORD { pub ExceptionCode : NTSTATUS , pub ExceptionFlags : u32 , pub ExceptionRecord : * mut EXCEPTION_RECORD , pub ExceptionAddress : * mut core :: ffi :: c_void , pub NumberParameters : u32 , pub ExceptionInformation : [usize ; 15] , }}} 
            }}

macro_rules! reserve_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reserve_stack in module {}", module_path!());
    };
}

mkfn!{
    reserve_stack_introspect!();
    # [doc = " Reserve stack space for use in stack overflow exceptions."] fn reserve_stack () { let result = unsafe { c :: SetThreadStackGuarantee (& mut 0x5000) } ; debug_assert_ne ! (result , 0 , "failed to reserve stack space for exception handling") ; }
}

macro_rules! vectored_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vectored_handler in module {}", module_path!());
    };
}

mkfn!{
    vectored_handler_introspect!();
    unsafe extern "system" fn vectored_handler (ExceptionInfo : * mut c :: EXCEPTION_POINTERS) -> i32 { unsafe { let rec = & (* (* ExceptionInfo) . ExceptionRecord) ; let code = rec . ExceptionCode ; if code == c :: EXCEPTION_STACK_OVERFLOW { crate :: thread :: with_current_name (| name | { let name = name . unwrap_or ("<unknown>") ; let tid = crate :: thread :: current_os_id () ; rtprintpanic ! ("\nthread '{name}' ({tid}) has overflowed its stack\n") ; }) ; } c :: EXCEPTION_CONTINUE_SEARCH } }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub unsafe fn init () { unsafe { let result = c :: AddVectoredExceptionHandler (0 , Some (vectored_handler)) ; debug_assert ! (! result . is_null () , "failed to install exception handler") ; } reserve_stack () ; }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { }
}

macro_rules! make_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_handler in module {}", module_path!());
    };
}

mkfn!{
    make_handler_introspect!();
    pub unsafe fn make_handler (main_thread : bool , _thread_name : Option < Box < str > > ,) -> super :: Handler { if ! main_thread { reserve_stack () ; } super :: Handler :: null () }
}

macro_rules! drop_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_handler in module {}", module_path!());
    };
}

mkfn!{
    drop_handler_introspect!();
    pub unsafe fn drop_handler (_data : * mut libc :: c_void) { }
} 
            }}