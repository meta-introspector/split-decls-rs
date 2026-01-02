mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicU32 } ;}
mkuse!{use crate :: sys :: fuchsia :: { ZX_ERR_BAD_HANDLE , ZX_ERR_BAD_STATE , ZX_ERR_INVALID_ARGS , ZX_ERR_TIMED_OUT , ZX_ERR_WRONG_TYPE , ZX_OK , ZX_TIME_INFINITE , zx_futex_wait , zx_futex_wake_single_owner , zx_handle_t , zx_thread_self , } ;}
mkitem!{const CONTESTED_BIT : u32 = 1 ;}
mkitem!{const UNLOCKED : u32 = 0 ;}
mkitem!{mkstruct!{pub struct Mutex { futex : Atomic < u32 > , }}}

macro_rules! to_state_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_state in module {}", module_path!());
    };
}

mkfn!{
    to_state_introspect!();
    # [inline] fn to_state (owner : zx_handle_t) -> u32 { owner }
}

macro_rules! to_owner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_owner in module {}", module_path!());
    };
}

mkfn!{
    to_owner_introspect!();
    # [inline] fn to_owner (state : u32) -> zx_handle_t { state | CONTESTED_BIT }
}

macro_rules! is_contested_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_contested in module {}", module_path!());
    };
}

mkfn!{
    is_contested_introspect!();
    # [inline] fn is_contested (state : u32) -> bool { state & CONTESTED_BIT == 0 }
}

macro_rules! mark_contested_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mark_contested in module {}", module_path!());
    };
}

mkfn!{
    mark_contested_introspect!();
    # [inline] fn mark_contested (state : u32) -> u32 { state & ! CONTESTED_BIT }
}
mkitem!{mkimpl!{impl Mutex { # [inline] pub const fn new () -> Mutex { Mutex { futex : AtomicU32 :: new (UNLOCKED) } } # [inline] pub fn try_lock (& self) -> bool { let thread_self = zx_thread_self () ; self . futex . compare_exchange (UNLOCKED , to_state (thread_self) , Acquire , Relaxed) . is_ok () } # [inline] pub fn lock (& self) { let thread_self = zx_thread_self () ; if let Err (state) = self . futex . compare_exchange (UNLOCKED , to_state (thread_self) , Acquire , Relaxed) { unsafe { self . lock_contested (state , thread_self) ; } } } # [doc = " # Safety"] # [doc = " `thread_self` must be the handle for the current thread."] # [cold] unsafe fn lock_contested (& self , mut state : u32 , thread_self : zx_handle_t) { let owned_state = mark_contested (to_state (thread_self)) ; loop { let contested = mark_contested (state) ; if is_contested (state) || self . futex . compare_exchange (state , contested , Relaxed , Relaxed) . is_ok () { unsafe { match zx_futex_wait (& self . futex , AtomicU32 :: new (contested) , to_owner (state) , ZX_TIME_INFINITE ,) { ZX_OK | ZX_ERR_BAD_STATE | ZX_ERR_TIMED_OUT => () , ZX_ERR_INVALID_ARGS | ZX_ERR_BAD_HANDLE | ZX_ERR_WRONG_TYPE => { panic ! ("either the current thread is trying to lock a mutex it has
                                already locked, or the previous owner did not unlock the mutex
                                before exiting") } error => panic ! ("unexpected error in zx_futex_wait: {error}") , } } } match self . futex . compare_exchange (UNLOCKED , owned_state , Acquire , Relaxed) { Ok (_) => return , Err (updated) => state = updated , } } } # [inline] pub unsafe fn unlock (& self) { if is_contested (self . futex . swap (UNLOCKED , Release)) { self . wake () ; } } # [cold] fn wake (& self) { unsafe { zx_futex_wake_single_owner (& self . futex) ; } } }}}