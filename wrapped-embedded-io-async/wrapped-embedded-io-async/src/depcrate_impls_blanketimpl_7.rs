// Generated macro for impl_7 (impl)
macro_rules! Depcrate_impls_blanketimpl_7 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : ? Sized + BufRead > BufRead for & mut T { # [inline] async fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { T :: fill_buf (self) . await } # [inline] fn consume (& mut self , amt : usize) { T :: consume (self , amt) ; } }
};
}
