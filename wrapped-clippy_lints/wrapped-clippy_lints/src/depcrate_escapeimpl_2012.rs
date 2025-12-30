// Generated macro for impl_2012 (impl)
macro_rules! Depcrate_escapeimpl_2012 {
() => {
// Module: crate::escape
// Provides: {"impl_2012"}
// Dependencies: {}
impl < 'tcx > EscapeDelegate < '_ , 'tcx > { fn is_large_box (& self , ty : Ty < 'tcx >) -> bool { if let Some (boxed_ty) = ty . boxed_ty () { self . cx . layout_of (boxed_ty) . map_or (0 , | l | l . size . bytes ()) > self . too_large_for_stack } else { false } } }
};
}
