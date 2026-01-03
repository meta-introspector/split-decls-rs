// rustc_index module stub
pub struct Idx;
pub struct IndexVec;
pub struct IndexSlice;
pub struct IntoSliceIdx;

pub mod bit_set {
    pub struct DenseBitSet;
    pub struct BitSet;
    pub struct GrowableBitSet;
    pub struct BitMatrix;
}

// Missing items from build log
pub mod static_assert_size {}

// Re-export commonly used items
pub use bit_set::*;
