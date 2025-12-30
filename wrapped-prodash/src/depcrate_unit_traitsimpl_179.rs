// Generated macro for impl_179 (impl)
macro_rules! Depcrate_unit_traitsimpl_179 {
() => {
// Module: crate::unit::traits
// Provides: {"impl_179"}
// Dependencies: {}
impl DisplayValue for & 'static str { fn dyn_hash (& self , state : & mut dyn Hasher) { state . write (self . as_bytes ()) } fn display_unit (& self , w : & mut dyn fmt :: Write , _value : usize) -> fmt :: Result { w . write_fmt (format_args ! ("{}" , self)) } }
};
}
