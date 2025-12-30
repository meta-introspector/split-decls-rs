// Generated macro for impl_68 (impl)
macro_rules! Depcrate_formattersimpl_68 {
() => {
// Module: crate::formatters
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , U : ToF64 + Unsigned + Copy , O : AsRef < FormatSizeOptions > > From < & 'a SizeFormatter < U , O > > for ISizeFormatter < U , & 'a O > { fn from (source : & 'a SizeFormatter < U , O >) -> Self { ISizeFormatter { value : source . value , options : & source . options , } } }
};
}
