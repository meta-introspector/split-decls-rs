// Generated macro for assoc_tag_str (function)
macro_rules! Depcrate_hir_ty_lowering_errorsassoc_tag_str {
() => {
// Module: crate::hir_ty_lowering::errors
// Provides: {"assoc_tag_str"}
// Dependencies: {}
pub (crate) fn assoc_tag_str (assoc_tag : ty :: AssocTag) -> & 'static str { match assoc_tag { ty :: AssocTag :: Fn => "function" , ty :: AssocTag :: Const => "constant" , ty :: AssocTag :: Type => "type" , } }
};
}
