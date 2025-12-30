// Generated macro for impl_173 (impl)
macro_rules! Depcrate_unit_rangeimpl_173 {
() => {
// Module: crate::unit::range
// Provides: {"impl_173"}
// Dependencies: {}
impl DisplayValue for Range { fn display_current_value (& self , w : & mut dyn fmt :: Write , value : Step , _upper : Option < Step >) -> fmt :: Result { w . write_fmt (format_args ! ("{}" , value + 1)) } fn separator (& self , w : & mut dyn fmt :: Write , _value : Step , _upper : Option < Step >) -> fmt :: Result { w . write_str (" of ") } fn dyn_hash (& self , state : & mut dyn Hasher) { self . name . dyn_hash (state) } fn display_unit (& self , w : & mut dyn fmt :: Write , _value : Step) -> fmt :: Result { w . write_str (self . name) } }
};
}
