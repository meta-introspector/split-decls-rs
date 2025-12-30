// Generated macro for impl_482 (impl)
macro_rules! Depcrateimpl_482 {
() => {
// Module: crate
// Provides: {"impl_482"}
// Dependencies: {}
impl CaptureUsages { pub fn sources (& self , db : & dyn HirDatabase) -> Vec < CaptureUsageSource > { let (body , source_map) = db . body_with_source_map (self . parent) ; let mut result = Vec :: with_capacity (self . spans . len ()) ; for & span in self . spans . iter () { let is_ref = span . is_ref_span (& body) ; match span { mir :: MirSpan :: ExprId (expr) => { if let Ok (expr) = source_map . expr_syntax (expr) { result . push (CaptureUsageSource { is_ref , source : expr }) } } mir :: MirSpan :: PatId (pat) => { if let Ok (pat) = source_map . pat_syntax (pat) { result . push (CaptureUsageSource { is_ref , source : pat }) ; } } mir :: MirSpan :: BindingId (binding) => result . extend (source_map . patterns_for_binding (binding) . iter () . filter_map (| & pat | source_map . pat_syntax (pat) . ok ()) . map (| pat | CaptureUsageSource { is_ref , source : pat }) ,) , mir :: MirSpan :: SelfParam | mir :: MirSpan :: Unknown => { unreachable ! ("invalid capture usage span") } } } result } }
};
}
