// Generated macro for apply_capture_kind_on_capture_ty (function)
macro_rules! Depcrate_upvarapply_capture_kind_on_capture_ty {
() => {
// Module: crate::upvar
// Provides: {"apply_capture_kind_on_capture_ty"}
// Dependencies: {}
# [doc = " Returns a Ty that applies the specified capture kind on the provided capture Ty"] fn apply_capture_kind_on_capture_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , capture_kind : UpvarCapture , region : ty :: Region < 'tcx > ,) -> Ty < 'tcx > { match capture_kind { ty :: UpvarCapture :: ByValue | ty :: UpvarCapture :: ByUse => ty , ty :: UpvarCapture :: ByRef (kind) => Ty :: new_ref (tcx , region , ty , kind . to_mutbl_lossy ()) , } }
};
}
