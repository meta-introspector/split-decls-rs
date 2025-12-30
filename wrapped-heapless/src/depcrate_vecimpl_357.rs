// Generated macro for impl_357 (impl)
macro_rules! Depcrate_vecimpl_357 {
() => {
// Module: crate::vec
// Provides: {"impl_357"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > hash :: Hash for VecInner < T , LenT , S > where T : core :: hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { < [T] as hash :: Hash > :: hash (self , state) ; } }
};
}
