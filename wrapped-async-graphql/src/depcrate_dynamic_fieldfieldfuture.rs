// Generated macro for FieldFuture (enum)
macro_rules! Depcrate_dynamic_fieldFieldFuture {
() => {
// Module: crate::dynamic::field
// Provides: {"FieldFuture"}
// Dependencies: {}
# [doc = " A future that returned from field resolver"] pub enum FieldFuture < 'a > { # [doc = " A pure value without any async operation"] Value (Option < FieldValue < 'a > >) , # [doc = " A future that returned from field resolver"] Future (BoxResolveFut < 'a >) , }
};
}
