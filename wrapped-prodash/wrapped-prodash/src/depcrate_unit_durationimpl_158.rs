// Generated macro for impl_158 (impl)
macro_rules! Depcrate_unit_durationimpl_158 {
() => {
// Module: crate::unit::duration
// Provides: {"impl_158"}
// Dependencies: {}
impl DisplayValue for Duration { fn display_current_value (& self , w : & mut dyn fmt :: Write , value : Step , _upper : Option < Step >) -> fmt :: Result { let dur = jiff :: SignedDuration :: from_secs (value as i64) ; w . write_str (& format ! ("{dur:#}")) } fn separator (& self , w : & mut dyn fmt :: Write , _value : Step , _upper : Option < Step >) -> fmt :: Result { w . write_str (" of ") } fn display_upper_bound (& self , w : & mut dyn fmt :: Write , upper_bound : Step , _value : Step) -> fmt :: Result { let dur = jiff :: SignedDuration :: from_secs (upper_bound as i64) ; w . write_str (& format ! ("{dur:#}")) } fn dyn_hash (& self , state : & mut dyn std :: hash :: Hasher) { state . write (& []) } fn display_unit (& self , _w : & mut dyn fmt :: Write , _value : Step) -> fmt :: Result { Ok (()) } }
};
}
