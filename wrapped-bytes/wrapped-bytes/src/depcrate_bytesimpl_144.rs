// Generated macro for impl_144 (impl)
macro_rules! Depcrate_bytesimpl_144 {
() => {
// Module: crate::bytes
// Provides: {"impl_144"}
// Dependencies: {}
impl PartialOrd < Vec < u8 > > for Bytes { fn partial_cmp (& self , other : & Vec < u8 >) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (& other [..]) } }
};
}
