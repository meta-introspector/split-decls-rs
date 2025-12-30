// Generated macro for check_attribute (function)
macro_rules! Depcrate_feature_gatecheck_attribute {
() => {
// Module: crate::feature_gate
// Provides: {"check_attribute"}
// Dependencies: {}
pub fn check_attribute (attr : & ast :: Attribute , sess : & Session , features : & Features) { PostExpansionVisitor { sess , features } . visit_attribute (attr) }
};
}
