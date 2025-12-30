// Generated macro for int_bitset (macro)
macro_rules! Depcrate_bitsint_bitset {
() => {
// Module: crate::bits
// Provides: {"int_bitset"}
// Dependencies: {}
macro_rules ! int_bitset { ($ typ : ty) => { impl BitSetLike for $ typ { fn new_bitset (_ : usize) -> Self { 0 } fn len (& self) -> usize { mem :: size_of ::<$ typ > () * 8 } fn test (& self , ix : usize) -> bool { 0 != (* self & ((1 as $ typ) << ix)) } fn set (& mut self , ix : usize) { * self |= (1 as $ typ) << ix ; } fn clear (& mut self , ix : usize) { * self &= ! ((1 as $ typ) << ix) ; } fn count (& self) -> usize { self . count_ones () as usize } } } ; }
};
}
