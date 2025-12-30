// Generated macro for impl_69 (impl)
macro_rules! Depcrate_dateimpl_69 {
() => {
// Module: crate::date
// Provides: {"impl_69"}
// Dependencies: {}
impl < Tz : TimeZone > hash :: Hash for Date < Tz > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . date . hash (state) } }
};
}
