// Generated macro for impl_150 (impl)
macro_rules! Depcrateimpl_150 {
() => {
// Module: crate
// Provides: {"impl_150"}
// Dependencies: {}
impl PosixDay { fn quote (& self) -> proc_macro2 :: TokenStream { match * self { PosixDay :: JulianOne (day) => quote ! { jiff :: shared :: PosixDay :: JulianOne (# day) } , PosixDay :: JulianZero (day) => quote ! { jiff :: shared :: PosixDay :: JulianZero (# day) } , PosixDay :: WeekdayOfMonth { month , week , weekday } => quote ! { jiff :: shared :: PosixDay :: WeekdayOfMonth { month : # month , week : # week , weekday : # weekday , } } , } } }
};
}
