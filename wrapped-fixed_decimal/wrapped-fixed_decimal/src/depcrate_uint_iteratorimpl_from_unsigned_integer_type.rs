// Generated macro for impl_from_unsigned_integer_type (macro)
macro_rules! Depcrate_uint_iteratorimpl_from_unsigned_integer_type {
() => {
// Module: crate::uint_iterator
// Provides: {"impl_from_unsigned_integer_type"}
// Dependencies: {}
macro_rules ! impl_from_unsigned_integer_type { ($ utype : ident) => { impl From <$ utype > for IntIterator <$ utype > { fn from (value : $ utype) -> Self { Self { unum : value , is_negative : false , } } } impl_iterator_unsigned_integer_type ! ($ utype) ; } ; }
};
}
