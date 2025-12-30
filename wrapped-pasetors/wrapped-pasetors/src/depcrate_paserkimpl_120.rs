// Generated macro for impl_120 (impl)
macro_rules! Depcrate_paserkimpl_120 {
() => {
// Module: crate::paserk
// Provides: {"impl_120"}
// Dependencies: {}
impl FormatAsPaserk for Id { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str (& self . header) ? ; write . write_str (& self . identifier) } }
};
}
