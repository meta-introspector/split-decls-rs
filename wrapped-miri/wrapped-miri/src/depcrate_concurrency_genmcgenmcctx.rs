// Generated macro for GenmcCtx (struct)
macro_rules! Depcrate_concurrency_genmcGenmcCtx {
() => {
// Module: crate::concurrency::genmc
// Provides: {"GenmcCtx"}
// Dependencies: {}
pub struct GenmcCtx { # [doc = " Some actions Miri does are allowed to cause data races."] # [doc = " GenMC will not be informed about certain actions (e.g. non-atomic loads) when this flag is set."] allow_data_races : Cell < bool > , }
};
}
