// Generated macro for impl_iterator_unsigned_integer_type (macro)
macro_rules! Depcrate_uint_iteratorimpl_iterator_unsigned_integer_type {
() => {
// Module: crate::uint_iterator
// Provides: {"impl_iterator_unsigned_integer_type"}
// Dependencies: {}
macro_rules ! impl_iterator_unsigned_integer_type { ($ utype : ident) => { impl Iterator for IntIterator <$ utype > { type Item = u8 ; fn next (& mut self) -> Option < Self :: Item > { if self . unum == 0 { None } else { let div = self . unum / 10 ; let rem = self . unum % 10 ; self . unum = div ; Some (rem as u8) } } } } ; }
};
}
