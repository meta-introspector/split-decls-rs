// Generated macro for impl_8 (impl)
macro_rules! Depcrate_impls_blanketimpl_8 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : ? Sized + BufRead > BufRead for & mut T { # [inline] fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { T :: fill_buf (self) } # [inline] fn consume (& mut self , amt : usize) { T :: consume (self , amt) ; } }
};
}
