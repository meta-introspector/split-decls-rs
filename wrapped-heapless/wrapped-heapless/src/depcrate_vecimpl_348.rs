// Generated macro for impl_348 (impl)
macro_rules! Depcrate_vecimpl_348 {
() => {
// Module: crate::vec
// Provides: {"impl_348"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > fmt :: Debug for VecInner < T , LenT , S > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < [T] as fmt :: Debug > :: fmt (self , f) } }
};
}
