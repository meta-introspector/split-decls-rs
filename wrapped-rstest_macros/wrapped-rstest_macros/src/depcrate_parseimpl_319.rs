// Generated macro for impl_319 (impl)
macro_rules! Depcrate_parseimpl_319 {
() => {
// Module: crate::parse
// Provides: {"impl_319"}
// Dependencies: {}
impl VisitMut for CheckTimeoutAttributesFunction { fn visit_item_fn_mut (& mut self , node : & mut ItemFn) { let timeouts = node . attrs . iter () . filter (| & a | attr_is (a , "timeout")) . collect :: < Vec < _ > > () ; let mut errors = timeouts . iter () . map (| & attr | attr . parse_args :: < syn :: Expr > ()) . filter_map (Result :: err) . collect :: < Vec < _ > > () ; if let Some (e) = self . check_if_can_implement_timeous (timeouts . as_slice () , node . sig . asyncness . as_ref ()) { errors . push (e) ; } if ! errors . is_empty () { * self = Self (Err (errors . into ())) ; } } }
};
}
