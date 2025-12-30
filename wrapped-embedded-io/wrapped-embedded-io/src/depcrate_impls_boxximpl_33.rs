// Generated macro for impl_33 (impl)
macro_rules! Depcrate_impls_boxximpl_33 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + BufRead > BufRead for Box < T > { # [inline] fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { T :: fill_buf (self) } # [inline] fn consume (& mut self , amt : usize) { T :: consume (self , amt) ; } }
};
}
