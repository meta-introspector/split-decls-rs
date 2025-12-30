// Generated macro for MainThreadState (enum)
macro_rules! Depcrate_evalMainThreadState {
() => {
// Module: crate::eval
// Provides: {"MainThreadState"}
// Dependencies: {}
# [doc = " The state of the main thread. Implementation detail of `on_main_stack_empty`."] # [derive (Debug)] enum MainThreadState < 'tcx > { GlobalCtors { ctor_state : global_ctor :: GlobalCtorState < 'tcx > , # [doc = " The main function to call."] entry_id : DefId , entry_type : MiriEntryFnType , # [doc = " Arguments passed to `main`."] argc : ImmTy < 'tcx > , argv : ImmTy < 'tcx > , } , Running , TlsDtors (tls :: TlsDtorsState < 'tcx >) , Yield { remaining : u32 , } , Done , }
};
}
