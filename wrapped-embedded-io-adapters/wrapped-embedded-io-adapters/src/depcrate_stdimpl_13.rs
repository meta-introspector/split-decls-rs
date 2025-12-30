// Generated macro for impl_13 (impl)
macro_rules! Depcrate_stdimpl_13 {
() => {
// Module: crate::std
// Provides: {"impl_13"}
// Dependencies: {}
# [deny (clippy :: missing_trait_methods , reason = "Methods should be forwarded to the underlying type")] impl < T : std :: io :: BufRead + ? Sized > embedded_io :: BufRead for FromStd < T > { fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
};
}
