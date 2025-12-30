// Generated macro for is_stable_diagnostic_attribute (function)
macro_rules! Depcrate_builtin_attrsis_stable_diagnostic_attribute {
() => {
// Module: crate::builtin_attrs
// Provides: {"is_stable_diagnostic_attribute"}
// Dependencies: {}
pub fn is_stable_diagnostic_attribute (sym : Symbol , _features : & Features) -> bool { match sym { sym :: on_unimplemented | sym :: do_not_recommend => true , _ => false , } }
};
}
