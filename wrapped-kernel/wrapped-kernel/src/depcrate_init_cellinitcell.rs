// Generated macro for InitCell (struct)
macro_rules! Depcrate_init_cellInitCell {
() => {
// Module: crate::init_cell
// Provides: {"InitCell"}
// Dependencies: {}
# [doc = " A cell for iteratively initializing a `OnceCell`."] # [doc = ""] # [doc = " This should be used as a stop-gap measure only."] pub struct InitCell < T > { init : SpinMutex < Option < T > > , once : OnceCell < T > , }
};
}
