// Generated macro for EvalContextExtPriv (trait)
macro_rules! Depcrate_shims_unix_macos_syncEvalContextExtPriv {
() => {
// Module: crate::shims::unix::macos::sync
// Provides: {"EvalContextExtPriv"}
// Dependencies: {}
trait EvalContextExtPriv < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn os_unfair_lock_get_data < 'a > (& 'a mut self , lock_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , & 'a MacOsUnfairLock > where 'tcx : 'a , { let this = self . eval_context_mut () ; let lock = this . deref_pointer_as (lock_ptr , this . libc_ty_layout ("os_unfair_lock_s")) ? ; this . lazy_sync_get_data (& lock , Size :: ZERO , | | { interp_ok (MacOsUnfairLock :: Poisoned) } , | _ | interp_ok (MacOsUnfairLock :: Active { mutex_ref : MutexRef :: new () }) ,) } }
};
}
