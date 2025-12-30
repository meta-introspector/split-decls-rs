// Generated macro for AllowReturnTypeNotation (enum)
macro_rules! DepcrateAllowReturnTypeNotation {
() => {
// Module: crate
// Provides: {"AllowReturnTypeNotation"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] enum AllowReturnTypeNotation { # [doc = " Only in types, since RTN is denied later during HIR lowering."] Yes , # [doc = " All other positions (path expr, method, use tree)."] No , }
};
}
