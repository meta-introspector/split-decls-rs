// Generated macro for TruncSide (enum)
macro_rules! Depcrate_block_apiTruncSide {
() => {
// Module: crate::block_api
// Provides: {"TruncSide"}
// Dependencies: {}
# [doc = " Type which used for defining truncation side in the [`VariableOutputCore`]"] # [doc = " trait."] # [derive (Copy , Clone , Debug)] pub enum TruncSide { # [doc = " Truncate left side, i.e. `&out[..n]`."] Left , # [doc = " Truncate right side, i.e. `&out[m..]`."] Right , }
};
}
