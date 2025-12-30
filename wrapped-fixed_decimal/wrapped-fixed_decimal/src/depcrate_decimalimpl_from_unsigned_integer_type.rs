// Generated macro for impl_from_unsigned_integer_type (macro)
macro_rules! Depcrate_decimalimpl_from_unsigned_integer_type {
() => {
// Module: crate::decimal
// Provides: {"impl_from_unsigned_integer_type"}
// Dependencies: {}
macro_rules ! impl_from_unsigned_integer_type { ($ utype : ident) => { impl From <$ utype > for UnsignedDecimal { fn from (value : $ utype) -> Self { let int_iterator : IntIterator <$ utype > = value . into () ; Self :: from_ascending (int_iterator) . expect ("All built-in integer types should fit") } } } ; }
};
}
