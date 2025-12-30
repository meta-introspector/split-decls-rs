// Generated macro for is_type_ref_to_diagnostic_item (function)
macro_rules! Depcrate_tyis_type_ref_to_diagnostic_item {
() => {
// Module: crate::ty
// Provides: {"is_type_ref_to_diagnostic_item"}
// Dependencies: {}
# [doc = " Checks if the type is a reference equals to a diagnostic item"] pub fn is_type_ref_to_diagnostic_item (cx : & LateContext < '_ > , ty : Ty < '_ > , diag_item : Symbol) -> bool { match ty . kind () { ty :: Ref (_ , ref_ty , _) => is_type_diagnostic_item (cx , * ref_ty , diag_item) , _ => false , } }
};
}
