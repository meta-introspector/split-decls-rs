// Generated macro for impl_44 (impl)
macro_rules! Depcrate_features_defmtimpl_44 {
() => {
// Module: crate::features::defmt
// Provides: {"impl_44"}
// Dependencies: {}
impl Format for ToCompactStringError { fn format (& self , fmt : defmt :: Formatter) { match self { ToCompactStringError :: Reserve (reserve_error) => reserve_error . format (fmt) , ToCompactStringError :: Fmt (core :: fmt :: Error) => { defmt :: write ! (fmt , "Display::fmt() returned an error") } } } }
};
}
