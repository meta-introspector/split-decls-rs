// Generated macro for impl_from_signed_integer_type (macro)
macro_rules! Depcrate_uint_iteratorimpl_from_signed_integer_type {
() => {
// Module: crate::uint_iterator
// Provides: {"impl_from_signed_integer_type"}
// Dependencies: {}
macro_rules ! impl_from_signed_integer_type { ($ itype : ident , $ utype : ident) => { impl From <$ itype > for IntIterator <$ utype > { fn from (value : $ itype) -> Self { Self { unum : { if value == $ itype :: MIN { $ itype :: MAX as $ utype + 1 } else { value . unsigned_abs () } } , is_negative : value . is_negative () , } } } } ; }
};
}
