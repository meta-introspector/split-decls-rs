mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sys :: thread_local :: key :: { LazyKey , set } ;}

macro_rules! enable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable in module {}", module_path!());
    };
}

mkfn!{
    enable_introspect!();
    # [cfg (target_thread_local)] pub fn enable () { use crate :: sys :: thread_local :: destructors ; static DTORS : LazyKey = LazyKey :: new (Some (run)) ; unsafe { set (DTORS . force () , ptr :: without_provenance_mut (1)) ; } unsafe extern "C" fn run (_ : * mut u8) { unsafe { destructors :: run () ; crate :: rt :: thread_cleanup () ; } } }
}

macro_rules! enable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable in module {}", module_path!());
    };
}

mkfn!{
    enable_introspect!();
    # [doc = " On platforms with key-based TLS, the system runs the destructors for us."] # [doc = " We still have to make sure that [`crate::rt::thread_cleanup`] is called,"] # [doc = " however. This is done by deferring the execution of a TLS destructor to"] # [doc = " the next round of destruction inside the TLS destructors."] # [cfg (not (target_thread_local))] pub fn enable () { const DEFER : * mut u8 = ptr :: without_provenance_mut (1) ; const RUN : * mut u8 = ptr :: without_provenance_mut (2) ; static CLEANUP : LazyKey = LazyKey :: new (Some (run)) ; unsafe { set (CLEANUP . force () , DEFER) } unsafe extern "C" fn run (state : * mut u8) { if state == DEFER { unsafe { set (CLEANUP . force () , RUN) } } else { debug_assert_eq ! (state , RUN) ; crate :: rt :: thread_cleanup () ; } } }
}