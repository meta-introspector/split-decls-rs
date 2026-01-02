mkuse!{use core :: arch :: asm ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: os :: xous :: ffi :: { MemoryFlags , Syscall , ThreadId , blocking_scalar , create_thread , do_yield , join_thread , map_memory , update_memory_flags , } ;}
mkuse!{use crate :: os :: xous :: services :: { TicktimerScalar , ticktimer_server } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Thread { tid : ThreadId , }}}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 131072 ;}
mkitem!{const MIN_STACK_SIZE : usize = 4096 ;}
mkitem!{pub const GUARD_PAGE_SIZE : usize = 4096 ;}
mkitem!{mkimpl!{impl Thread { pub unsafe fn new (stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { let p = Box :: into_raw (Box :: new (p)) ; let mut stack_size = crate :: cmp :: max (stack , MIN_STACK_SIZE) ; if (stack_size & 4095) != 0 { stack_size = (stack_size + 4095) & ! 4095 ; } let stack_plus_guard_pages : & mut [u8] = unsafe { map_memory (None , None , GUARD_PAGE_SIZE + stack_size + GUARD_PAGE_SIZE , MemoryFlags :: R | MemoryFlags :: W | MemoryFlags :: X ,) } . map_err (| code | io :: Error :: from_raw_os_error (code as i32)) ? ; unsafe { update_memory_flags (& mut stack_plus_guard_pages [0 .. GUARD_PAGE_SIZE] , MemoryFlags :: W) . map_err (| code | io :: Error :: from_raw_os_error (code as i32)) ? } ; unsafe { update_memory_flags (& mut stack_plus_guard_pages [(GUARD_PAGE_SIZE + stack_size) ..] , MemoryFlags :: W ,) . map_err (| code | io :: Error :: from_raw_os_error (code as i32)) ? } ; let guard_page_pre = stack_plus_guard_pages . as_ptr () as usize ; let tid = create_thread (thread_start as * mut usize , & mut stack_plus_guard_pages [GUARD_PAGE_SIZE .. (stack_size + GUARD_PAGE_SIZE)] , p as usize , guard_page_pre , stack_size , 0 ,) . map_err (| code | io :: Error :: from_raw_os_error (code as i32)) ? ; extern "C" fn thread_start (main : * mut usize , guard_page_pre : usize , stack_size : usize ,) -> ! { unsafe { Box :: from_raw (main as * mut Box < dyn FnOnce () >) () ; } unsafe { crate :: sys :: thread_local :: key :: destroy_tls () ; } let mapped_memory_base = guard_page_pre ; let mapped_memory_length = GUARD_PAGE_SIZE + stack_size + GUARD_PAGE_SIZE ; unsafe { asm ! ("ecall" , "ret" , in ("a0") Syscall :: UnmapMemory as usize , in ("a1") mapped_memory_base , in ("a2") mapped_memory_length , in ("ra") 0xff80_3000usize , options (nomem , nostack , noreturn)) ; } } Ok (Thread { tid }) } pub fn join (self) { join_thread (self . tid) . unwrap () ; } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { Ok (unsafe { NonZero :: new_unchecked (1) }) }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { do_yield () ; }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { let mut millis = dur . as_millis () ; while millis > 0 { let sleep_duration = if millis > (usize :: MAX as _) { usize :: MAX } else { millis as usize } ; blocking_scalar (ticktimer_server () , TicktimerScalar :: SleepMs (sleep_duration) . into ()) . expect ("failed to send message to ticktimer server") ; millis -= sleep_duration as u128 ; } }
}