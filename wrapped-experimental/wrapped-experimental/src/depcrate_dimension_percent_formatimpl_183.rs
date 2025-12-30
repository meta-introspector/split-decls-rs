// Generated macro for impl_183 (impl)
macro_rules! Depcrate_dimension_percent_formatimpl_183 {
() => {
// Module: crate::dimension::percent::format
// Provides: {"impl_183"}
// Dependencies: {}
impl Writeable for FormattedPercent < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { let abs_value = match self . value . sign () { Sign :: Negative => self . value . clone () . with_sign (Sign :: None) , _ => self . value . to_owned () , } ; let value = self . decimal_formatter . format (& abs_value) ; match self . options . display { Display :: Standard => { if self . value . sign () == Sign :: Negative { self . essential . signed_pattern . interpolate ((value , & self . essential . minus_sign)) . write_to (sink) ? } else { self . essential . unsigned_pattern . interpolate ([value]) . write_to (sink) ? } ; } Display :: Approximate => { let sign = if self . value . sign () == Sign :: Negative { Append (& self . essential . approximately_sign , & self . essential . minus_sign ,) } else { Append (& self . essential . approximately_sign , & Cow :: Borrowed ("")) } ; self . essential . signed_pattern . interpolate ((value , sign)) . write_to (sink) ? ; } Display :: ExplicitSign => self . essential . signed_pattern . interpolate ((value , if self . value . sign () == Sign :: Negative { & self . essential . minus_sign } else { & self . essential . plus_sign } ,)) . write_to (sink) ? , } ; Ok (()) } }
};
}
