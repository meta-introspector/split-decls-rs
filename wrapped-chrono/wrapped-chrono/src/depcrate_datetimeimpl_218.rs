// Generated macro for impl_218 (impl)
macro_rules! Depcrate_datetimeimpl_218 {
() => {
// Module: crate::datetime
// Provides: {"impl_218"}
// Dependencies: {}
impl < Tz : TimeZone > hash :: Hash for DateTime < Tz > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . datetime . hash (state) } }
};
}
