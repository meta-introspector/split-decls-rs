// Generated macro for impl_181 (impl)
macro_rules! Depcrate_dimension_percent_formatimpl_181 {
() => {
// Module: crate::dimension::percent::format
// Provides: {"impl_181"}
// Dependencies: {}
impl < W1 : Writeable , W2 : Writeable > Writeable for Append < W1 , W2 > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { self . 0 . write_to (sink) ? ; self . 1 . write_to (sink) } }
};
}
