// Generated macro for impl_131 (impl)
macro_rules! Depcrate_gridimpl_131 {
() => {
// Module: crate::grid
// Provides: {"impl_131"}
// Dependencies: {}
impl Properties { # [doc = " Hides the gridlines"] # [doc = ""] # [doc = " **Note** Both `Major` and `Minor` gridlines are hidden by default"] pub fn hide (& mut self) -> & mut Properties { self . hidden = true ; self } # [doc = " Shows the gridlines"] pub fn show (& mut self) -> & mut Properties { self . hidden = false ; self } }
};
}
