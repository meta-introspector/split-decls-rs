// Generated macro for impl_39 (impl)
macro_rules! Depcrate_digestimpl_39 {
() => {
// Module: crate::digest
// Provides: {"impl_39"}
// Dependencies: {}
impl Debug for Digest16 { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Digest16(") ? ; write_hex (f , self . 0 . as_slice ()) ? ; write ! (f , ")") } }
};
}
