// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
# [doc = " Return **true** if the bit is enabled in the bitset,"] # [doc = " or **false** otherwise."] # [doc = ""] # [doc = " Note: bits outside the capacity are always disabled, and thus"] # [doc = " indexing a FixedBitSet will not panic."] impl Index < usize > for FixedBitSet { type Output = bool ; # [inline] fn index (& self , bit : usize) -> & bool { if self . contains (bit) { & true } else { & false } } }
};
}
