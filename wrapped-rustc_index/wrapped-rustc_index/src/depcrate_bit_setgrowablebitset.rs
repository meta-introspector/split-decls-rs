// Generated macro for GrowableBitSet (struct)
macro_rules! Depcrate_bit_setGrowableBitSet {
() => {
// Module: crate::bit_set
// Provides: {"GrowableBitSet"}
// Dependencies: {}
# [doc = " A resizable bitset type with a dense representation."] # [doc = ""] # [doc = " `T` is an index type, typically a newtyped `usize` wrapper, but it can also"] # [doc = " just be `usize`."] # [doc = ""] # [doc = " All operations that involve an element will panic if the element is equal"] # [doc = " to or greater than the domain size."] # [derive (Clone , Debug , PartialEq)] pub struct GrowableBitSet < T : Idx > { bit_set : DenseBitSet < T > , }
};
}
