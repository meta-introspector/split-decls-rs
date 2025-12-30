// Generated macro for get_spans (function)
macro_rules! Depcrate_ptrget_spans {
() => {
// Module: crate::ptr
// Provides: {"get_spans"}
// Dependencies: {}
pub fn get_spans (cx : & LateContext < '_ > , opt_body_id : Option < BodyId > , idx : usize , replacements : & [(Symbol , & 'static str)] ,) -> Option < Vec < (Span , Cow < 'static , str >) > > { if let Some (body) = opt_body_id . map (| id | cx . tcx . hir_body (id)) { if let PatKind :: Binding (_ , binding_id , _ , _) = strip_pat_refs (body . params [idx] . pat) . kind { extract_clone_suggestions (cx , binding_id , replacements , body) } else { Some (vec ! []) } } else { Some (vec ! []) } }
};
}
