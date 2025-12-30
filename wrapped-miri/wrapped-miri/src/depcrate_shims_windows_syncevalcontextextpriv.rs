// Generated macro for EvalContextExtPriv (trait)
macro_rules! Depcrate_shims_windows_syncEvalContextExtPriv {
() => {
// Module: crate::shims::windows::sync
// Provides: {"EvalContextExtPriv"}
// Dependencies: {}
trait EvalContextExtPriv < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn init_once_get_data < 'a > (& 'a mut self , init_once_ptr : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , & 'a WindowsInitOnce > where 'tcx : 'a , { let this = self . eval_context_mut () ; let init_once = this . deref_pointer_as (init_once_ptr , this . windows_ty_layout ("INIT_ONCE")) ? ; let init_offset = Size :: ZERO ; this . lazy_sync_get_data (& init_once , init_offset , | | throw_ub_format ! ("`INIT_ONCE` can't be moved after first use") , | _ | { interp_ok (WindowsInitOnce { init_once : InitOnceRef :: new () }) } ,) } # [doc = " Returns `true` if we were succssful, `false` if we would block."] fn init_once_try_begin (& mut self , init_once_ref : & InitOnceRef , pending_place : & MPlaceTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , bool > { let this = self . eval_context_mut () ; interp_ok (match init_once_ref . status () { InitOnceStatus :: Uninitialized => { init_once_ref . begin () ; this . write_scalar (this . eval_windows ("c" , "TRUE") , pending_place) ? ; this . write_scalar (this . eval_windows ("c" , "TRUE") , dest) ? ; true } InitOnceStatus :: Complete => { this . init_once_observe_completed (init_once_ref) ; this . write_scalar (this . eval_windows ("c" , "FALSE") , pending_place) ? ; this . write_scalar (this . eval_windows ("c" , "TRUE") , dest) ? ; true } InitOnceStatus :: Begun => false , }) } }
};
}
