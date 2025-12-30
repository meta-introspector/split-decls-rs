// Generated macro for impl_from_signed_integer_type (macro)
macro_rules! Depcrate_signed_decimalimpl_from_signed_integer_type {
() => {
// Module: crate::signed_decimal
// Provides: {"impl_from_signed_integer_type"}
// Dependencies: {}
macro_rules ! impl_from_signed_integer_type { ($ itype : ident , $ utype : ident) => { impl From <$ itype > for Decimal { fn from (value : $ itype) -> Self { let int_iterator : IntIterator <$ utype > = value . into () ; let sign = if int_iterator . is_negative { Sign :: Negative } else { Sign :: None } ; let value = UnsignedDecimal :: from_ascending (int_iterator) . expect ("All built-in integer types should fit") ; Decimal { sign , absolute : value , } } } } ; }
};
}
