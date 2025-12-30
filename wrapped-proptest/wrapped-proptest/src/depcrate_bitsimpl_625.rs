// Generated macro for impl_625 (impl)
macro_rules! Depcrate_bitsimpl_625 {
() => {
// Module: crate::bits
// Provides: {"impl_625"}
// Dependencies: {}
impl BitSetLike for Vec < bool > { fn new_bitset (max : usize) -> Self { vec ! [false ; max] } fn len (& self) -> usize { self . len () } fn test (& self , bit : usize) -> bool { if bit >= self . len () { false } else { self [bit] } } fn set (& mut self , bit : usize) { if bit >= self . len () { self . resize (bit + 1 , false) ; } self [bit] = true ; } fn clear (& mut self , bit : usize) { if bit < self . len () { self [bit] = false ; } } fn count (& self) -> usize { self . iter () . filter (| & & b | b) . count () } }
};
}
