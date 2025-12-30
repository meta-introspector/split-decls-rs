// Generated macro for impl_156 (impl)
macro_rules! Depcrate_bytesimpl_156 {
() => {
// Module: crate::bytes
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a , T : ? Sized > PartialOrd < & 'a T > for Bytes where Bytes : PartialOrd < T > , { fn partial_cmp (& self , other : & & 'a T) -> Option < cmp :: Ordering > { self . partial_cmp (& * * other) } }
};
}
