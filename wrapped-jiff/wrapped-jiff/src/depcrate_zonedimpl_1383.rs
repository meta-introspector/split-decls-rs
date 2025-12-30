// Generated macro for impl_1383 (impl)
macro_rules! Depcrate_zonedimpl_1383 {
() => {
// Module: crate::zoned
// Provides: {"impl_1383"}
// Dependencies: {}
impl core :: hash :: Hash for Zoned { # [inline] fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . timestamp () . hash (state) ; } }
};
}
