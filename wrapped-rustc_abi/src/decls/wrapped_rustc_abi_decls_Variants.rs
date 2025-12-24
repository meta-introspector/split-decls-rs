use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub enum Variants<FieldIdx: Idx, VariantIdx: Idx> {
    /// A type with no valid variants. Must be uninhabited.
    Empty,
    /// Single enum variants, structs/tuples, unions, and all non-ADTs.
    Single {
        /// Always `0` for types that cannot have multiple variants.
        index: VariantIdx,
    },
    /// Enum-likes with more than one variant: each variant comes with
    /// a *discriminant* (usually the same as the variant index but the user can
    /// assign explicit discriminant values). That discriminant is encoded
    /// as a *tag* on the machine. The layout of each variant is
    /// a struct, and they all have space reserved for the tag.
    /// For enums, the tag is the sole field of the layout.
    Multiple {
        tag: Scalar,
        tag_encoding: TagEncoding<VariantIdx>,
        tag_field: FieldIdx,
        variants: IndexVec<VariantIdx, LayoutData<FieldIdx, VariantIdx>>,
    },
}
