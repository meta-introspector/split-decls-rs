mkuse!{use std :: alloc :: { Layout , alloc } ;}
mkuse!{use std :: { fmt , mem , ptr , slice } ;}
mkuse!{use rustc_interface :: util :: { DEFAULT_STACK_SIZE , STACK_SIZE } ;}
mkitem!{# [doc = " Signals that represent that we have a bug, and our prompt termination has"] # [doc = " been ordered."] # [rustfmt :: skip] const KILL_SIGNALS : [(libc :: c_int , & str) ; 3] = [(libc :: SIGILL , "SIGILL") , (libc :: SIGBUS , "SIGBUS") , (libc :: SIGSEGV , "SIGSEGV")] ;}
mkitem!{unsafe extern "C" { fn backtrace_symbols_fd (buffer : * const * mut libc :: c_void , size : libc :: c_int , fd : libc :: c_int) ; }}

macro_rules! backtrace_stderr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function backtrace_stderr in module {}", module_path!());
    };
}

mkfn!{
    backtrace_stderr_introspect!();
    fn backtrace_stderr (buffer : & [* mut libc :: c_void]) { let size = buffer . len () . try_into () . unwrap_or_default () ; unsafe { backtrace_symbols_fd (buffer . as_ptr () , size , libc :: STDERR_FILENO) } ; }
}
mkitem!{mkstruct!{# [doc = " Unbuffered, unsynchronized writer to stderr."] # [doc = ""] # [doc = " Only acceptable because everything will end soon anyways."] struct RawStderr (()) ;}}
mkitem!{mkimpl!{impl fmt :: Write for RawStderr { fn write_str (& mut self , s : & str) -> Result < () , fmt :: Error > { let ret = unsafe { libc :: write (libc :: STDERR_FILENO , s . as_ptr () . cast () , s . len ()) } ; if ret == - 1 { Err (fmt :: Error) } else { Ok (()) } } }}}
mkitem!{# [doc = " We don't really care how many bytes we actually get out. SIGSEGV comes for our head."] # [doc = " Splash stderr with letters of our own blood to warn our friends about the monster."] macro raw_errln ($ tokens : tt) { let _ = :: core :: fmt :: Write :: write_fmt (& mut RawStderr (()) , format_args ! ($ tokens)) ; let _ = :: core :: fmt :: Write :: write_char (& mut RawStderr (()) , '\n') ; }}

macro_rules! print_stack_trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_stack_trace in module {}", module_path!());
    };
}

mkfn!{
    print_stack_trace_introspect!();
    # [doc = " Signal handler installed for SIGSEGV"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Caller must ensure that this function is not re-entered."] unsafe extern "C" fn print_stack_trace (signum : libc :: c_int) { const MAX_FRAMES : usize = 256 ; let signame = { let mut signame = "<unknown>" ; for sig in KILL_SIGNALS { if sig . 0 == signum { signame = sig . 1 ; } } signame } ; let stack = unsafe { static mut STACK_TRACE : [* mut libc :: c_void ; MAX_FRAMES] = [ptr :: null_mut () ; MAX_FRAMES] ; let depth = libc :: backtrace (& raw mut STACK_TRACE as _ , MAX_FRAMES as i32) ; if depth == 0 { return ; } slice :: from_raw_parts (& raw const STACK_TRACE as _ , depth as _) } ; raw_errln ! ("error: rustc interrupted by {signame}, printing backtrace\n") ; let mut written = 1 ; let mut consumed = 0 ; let cycled = | (runner , walker) | runner == walker ; let mut cyclic = false ; if let Some (period) = stack . iter () . skip (1) . step_by (2) . zip (stack) . position (cycled) { let period = period . saturating_add (1) ; let Some (offset) = stack . iter () . skip (period) . zip (stack) . position (cycled) else { return ; } ; let next_cycle = stack [offset ..] . chunks_exact (period) . skip (1) ; let cycles = 1 + next_cycle . zip (stack [offset ..] . chunks_exact (period)) . filter (| (next , prev) | next == prev) . count () ; backtrace_stderr (& stack [.. offset]) ; written += offset ; consumed += offset ; if cycles > 1 { raw_errln ! ("\n### cycle encountered after {offset} frames with period {period}") ; backtrace_stderr (& stack [consumed .. consumed + period]) ; raw_errln ! ("### recursed {cycles} times\n") ; written += period + 4 ; consumed += period * cycles ; cyclic = true ; } ; } let rem = & stack [consumed ..] ; backtrace_stderr (rem) ; raw_errln ! ("") ; written += rem . len () + 1 ; let random_depth = | | 8 * 16 ; if (cyclic || stack . len () > random_depth ()) && signum == libc :: SIGSEGV { raw_errln ! ("note: rustc unexpectedly overflowed its stack! this is a bug") ; written += 1 ; } if stack . len () == MAX_FRAMES { raw_errln ! ("note: maximum backtrace depth reached, frames may have been lost") ; written += 1 ; } raw_errln ! ("note: we would appreciate a report at https://github.com/rust-lang/rust") ; written += 1 ; if signum == libc :: SIGSEGV { let new_size = STACK_SIZE . get () . copied () . unwrap_or (DEFAULT_STACK_SIZE) * 2 ; raw_errln ! ("help: you can increase rustc's stack size by setting RUST_MIN_STACK={new_size}") ; written += 1 ; } if written > 24 { raw_errln ! ("note: backtrace dumped due to {signame}! resuming signal") ; } ; }
}

macro_rules! install_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install in module {}", module_path!());
    };
}

mkfn!{
    install_introspect!();
    # [doc = " When one of the KILL signals is delivered to the process, print a stack trace and then exit."] pub (super) fn install () { unsafe { let alt_stack_size : usize = min_sigstack_size () + 64 * 1024 ; let mut alt_stack : libc :: stack_t = mem :: zeroed () ; alt_stack . ss_sp = alloc (Layout :: from_size_align (alt_stack_size , 1) . unwrap ()) . cast () ; alt_stack . ss_size = alt_stack_size ; libc :: sigaltstack (& alt_stack , ptr :: null_mut ()) ; let mut sa : libc :: sigaction = mem :: zeroed () ; sa . sa_sigaction = print_stack_trace as libc :: sighandler_t ; sa . sa_flags = libc :: SA_NODEFER | libc :: SA_RESETHAND | libc :: SA_ONSTACK ; libc :: sigemptyset (& mut sa . sa_mask) ; for (signum , _signame) in KILL_SIGNALS { libc :: sigaction (signum , & sa , ptr :: null_mut ()) ; } } }
}

macro_rules! min_sigstack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function min_sigstack_size in module {}", module_path!());
    };
}

mkfn!{
    min_sigstack_size_introspect!();
    # [doc = " Modern kernels on modern hardware can have dynamic signal stack sizes."] # [cfg (any (target_os = "linux" , target_os = "android"))] fn min_sigstack_size () -> usize { const AT_MINSIGSTKSZ : core :: ffi :: c_ulong = 51 ; let dynamic_sigstksz = unsafe { libc :: getauxval (AT_MINSIGSTKSZ) } ; libc :: MINSIGSTKSZ . max (dynamic_sigstksz as _) }
}

macro_rules! min_sigstack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function min_sigstack_size in module {}", module_path!());
    };
}

mkfn!{
    min_sigstack_size_introspect!();
    # [doc = " Not all OS support hardware where this is needed."] # [cfg (not (any (target_os = "linux" , target_os = "android")))] fn min_sigstack_size () -> usize { libc :: MINSIGSTKSZ }
}