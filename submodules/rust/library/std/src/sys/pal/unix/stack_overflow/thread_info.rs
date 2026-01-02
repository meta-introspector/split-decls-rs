mkuse!{use crate :: collections :: BTreeMap ;}
mkuse!{use crate :: hint :: spin_loop ;}
mkuse!{use crate :: ops :: Range ;}
mkuse!{use crate :: sync :: Mutex ;}
mkuse!{use crate :: sync :: atomic :: { AtomicUsize , Ordering } ;}
mkuse!{use crate :: sys :: os :: errno_location ;}
mkitem!{mkstruct!{pub struct ThreadInfo { pub guard_page_range : Range < usize > , pub thread_name : Option < Box < str > > , }}}
mkitem!{static LOCK : Mutex < () > = Mutex :: new (()) ;}
mkitem!{static SPIN_LOCK : AtomicUsize = AtomicUsize :: new (0) ;}
mkitem!{static mut THREAD_INFO : BTreeMap < usize , ThreadInfo > = BTreeMap :: new () ;}
mkitem!{mkstruct!{struct UnlockOnDrop ;}}
mkitem!{mkimpl!{impl Drop for UnlockOnDrop { fn drop (& mut self) { SPIN_LOCK . store (0 , Ordering :: Release) ; } }}}

macro_rules! with_current_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_current_info in module {}", module_path!());
    };
}

mkfn!{
    with_current_info_introspect!();
    # [doc = " Get the current thread's information, if available."] # [doc = ""] # [doc = " Calling this function might freeze other threads if they attempt to modify"] # [doc = " their thread information. Thus, the caller should ensure that the process"] # [doc = " is aborted shortly after this function is called."] # [doc = ""] # [doc = " This function is guaranteed to be async-signal-safe if `f` is too."] pub fn with_current_info < R > (f : impl FnOnce (Option < & ThreadInfo >) -> R) -> R { let this = errno_location () . addr () ; let mut attempt = 0 ; let _guard = loop { if attempt == 10_000_000 { rtprintpanic ! ("deadlock in SIGSEGV handler") ; return f (None) ; } match SPIN_LOCK . compare_exchange (0 , this , Ordering :: Acquire , Ordering :: Relaxed) { Ok (_) => break UnlockOnDrop , Err (owner) if owner == this => { rtabort ! ("a thread received SIGSEGV while modifying its stack overflow information") } Err (_) => { spin_loop () ; attempt += 1 ; } } } ; let thread_info = unsafe { & * (& raw const THREAD_INFO) } ; f (thread_info . get (& this)) }
}

macro_rules! spin_lock_in_setup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spin_lock_in_setup in module {}", module_path!());
    };
}

mkfn!{
    spin_lock_in_setup_introspect!();
    fn spin_lock_in_setup (this : usize) -> UnlockOnDrop { loop { match SPIN_LOCK . compare_exchange (0 , this , Ordering :: Acquire , Ordering :: Relaxed) { Ok (_) => return UnlockOnDrop , Err (owner) if owner == this => { unreachable ! ("the thread info setup logic isn't recursive") } Err (_) => drop (unsafe { libc :: pause () }) , } } }
}

macro_rules! set_current_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_current_info in module {}", module_path!());
    };
}

mkfn!{
    set_current_info_introspect!();
    pub fn set_current_info (guard_page_range : Range < usize > , thread_name : Option < Box < str > >) { let this = errno_location () . addr () ; let _lock_guard = LOCK . lock () ; let _spin_guard = spin_lock_in_setup (this) ; let thread_info = unsafe { & mut * (& raw mut THREAD_INFO) } ; thread_info . insert (this , ThreadInfo { guard_page_range , thread_name }) ; }
}

macro_rules! delete_current_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function delete_current_info in module {}", module_path!());
    };
}

mkfn!{
    delete_current_info_introspect!();
    pub fn delete_current_info () { let this = errno_location () . addr () ; let _lock_guard = LOCK . lock () ; let _spin_guard = spin_lock_in_setup (this) ; let thread_info = unsafe { & mut * (& raw mut THREAD_INFO) } ; thread_info . remove (& this) ; }
}