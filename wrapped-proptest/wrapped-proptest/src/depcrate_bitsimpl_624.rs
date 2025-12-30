// Generated macro for impl_624 (impl)
macro_rules! Depcrate_bitsimpl_624 {
() => {
// Module: crate::bits
// Provides: {"impl_624"}
// Dependencies: {}
# [cfg (feature = "bit-set")] # [cfg_attr (docsrs , doc (cfg (feature = "bit-set")))] impl BitSetLike for BitSet { fn new_bitset (max : usize) -> Self { BitSet :: with_capacity (max) } fn len (& self) -> usize { self . capacity () } fn test (& self , bit : usize) -> bool { self . contains (bit) } fn set (& mut self , bit : usize) { self . insert (bit) ; } fn clear (& mut self , bit : usize) { self . remove (bit) ; } fn count (& self) -> usize { self . len () } }
};
}
