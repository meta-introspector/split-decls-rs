// Generated macro for ConstantSource (enum)
macro_rules! Depcrate_constsConstantSource {
() => {
// Module: crate::consts
// Provides: {"ConstantSource"}
// Dependencies: {}
# [doc = " The source of a constant value."] # [derive (Clone , Copy)] pub enum ConstantSource { # [doc = " The value is determined solely from the expression."] Local , # [doc = " The value is dependent on another definition that may change independently from the local"] # [doc = " expression."] NonLocal , }
};
}
