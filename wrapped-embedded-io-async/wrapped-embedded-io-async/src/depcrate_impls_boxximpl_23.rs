// Generated macro for impl_23 (impl)
macro_rules! Depcrate_impls_boxximpl_23 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + BufRead > BufRead for Box < T > { # [inline] async fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { T :: fill_buf (self) . await } # [inline] fn consume (& mut self , amt : usize) { T :: consume (self , amt) ; } }
};
}
