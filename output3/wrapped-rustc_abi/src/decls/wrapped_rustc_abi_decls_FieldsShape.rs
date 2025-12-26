use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Describes how the fields of a type are located in memory.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub enum FieldsShape<FieldIdx: Idx> {
    /// Scalar primitives and `!`, which never have fields.
    Primitive,
    /// All fields start at no offset. The `usize` is the field count.
    Union(NonZeroUsize),
    /// Array/vector-like placement, with all fields of identical types.
    Array { stride: Size, count: u64 },
    /// Struct-like placement, with precomputed offsets.
    ///
    /// Fields are guaranteed to not overlap, but note that gaps
    /// before, between and after all the fields are NOT always
    /// padding, and as such their contents may not be discarded.
    /// For example, enum variants leave a gap at the start,
    /// where the discriminant field in the enum layout goes.
    Arbitrary {
        /// Offsets for the first byte of each field,
        /// ordered to match the source definition order.
        /// This vector does not go in increasing order.
        offsets: IndexVec<FieldIdx, Size>,
        /// Maps source order field indices to memory order indices,
        /// depending on how the fields were reordered (if at all).
        /// This is a permutation, with both the source order and the
        /// memory order using the same (0..n) index ranges.
        ///
        /// Note that during computation of `memory_index`, sometimes
        /// it is easier to operate on the inverse mapping (that is,
        /// from memory order to source order), and that is usually
        /// named `inverse_memory_index`.
        ///
        memory_index: IndexVec<FieldIdx, u32>,
    },
}
