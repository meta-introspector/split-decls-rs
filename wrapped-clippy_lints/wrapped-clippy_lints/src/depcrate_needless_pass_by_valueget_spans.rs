// Generated macro for get_spans (function)
macro_rules! Depcrate_needless_pass_by_valueget_spans {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"get_spans"}
// Dependencies: {}
fn get_spans < 'tcx > (cx : & LateContext < 'tcx > , body : & 'tcx Body < '_ > , idx : usize , replacements : & [(Symbol , & 'static str)] ,) -> Option < Vec < (Span , Cow < 'static , str >) > > { if let PatKind :: Binding (_ , binding_id , _ , _) = strip_pat_refs (body . params [idx] . pat) . kind { extract_clone_suggestions (cx , binding_id , replacements , body) } else { Some (vec ! []) } }
};
}
