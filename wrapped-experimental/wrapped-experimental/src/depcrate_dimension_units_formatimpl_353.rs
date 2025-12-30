// Generated macro for impl_353 (impl)
macro_rules! Depcrate_dimension_units_formatimpl_353 {
() => {
// Module: crate::dimension::units::format
// Provides: {"impl_353"}
// Dependencies: {}
impl Writeable for FormattedUnit < '_ > { fn write_to_parts < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : writeable :: PartsWrite + ? Sized , { self . display_name . patterns . get (self . value . into () , self . plural_rules) . interpolate ((self . decimal_formatter . format (self . value) ,)) . write_to_parts (sink) } }
};
}
