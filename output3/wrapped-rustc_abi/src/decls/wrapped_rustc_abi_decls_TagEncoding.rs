use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub enum TagEncoding<VariantIdx: Idx> {
    /// The tag directly stores the discriminant, but possibly with a smaller layout
    /// (so converting the tag to the discriminant can require sign extension).
    Direct,
    /// Niche (values invalid for a type) encoding the discriminant.
    /// Note that for this encoding, the discriminant and variant index of each variant coincide!
    /// This invariant is codified as part of [`layout_sanity_check`](../rustc_ty_utils/layout/invariant/fn.layout_sanity_check.html).
    ///
    /// The variant `untagged_variant` contains a niche at an arbitrary
    /// offset (field [`Variants::Multiple::tag_field`] of the enum).
    /// For a variant with variant index `i`, such that `i != untagged_variant`,
    /// the tag is set to `(i - niche_variants.start).wrapping_add(niche_start)`
    /// (this is wrapping arithmetic using the type of the niche field, cf. the
    /// [`tag_for_variant`](../rustc_const_eval/interpret/struct.InterpCx.html#method.tag_for_variant)
    /// query implementation).
    /// To recover the variant index `i` from a `tag`, the above formula has to be reversed,
    /// i.e. `i = tag.wrapping_sub(niche_start) + niche_variants.start`. If `i` ends up outside
    /// `niche_variants`, the tag must have encoded the `untagged_variant`.
    ///
    /// For example, `Option<(usize, &T)>`  is represented such that the tag for
    /// `None` is the null pointer in the second tuple field, and
    /// `Some` is the identity function (with a non-null reference)
    /// and has no additional tag, i.e. the reference being non-null uniquely identifies this variant.
    ///
    /// Other variants that are not `untagged_variant` and that are outside the `niche_variants`
    /// range cannot be represented; they must be uninhabited.
    /// Nonetheless, uninhabited variants can also fall into the range of `niche_variants`.
    Niche {
        untagged_variant: VariantIdx,
        /// This range *may* contain `untagged_variant` or uninhabited variants;
        /// these are then just "dead values" and not used to encode anything.
        niche_variants: RangeInclusive<VariantIdx>,
        /// This is inbounds of the type of the niche field
        /// (not sign-extended, i.e., all bits beyond the niche field size are 0).
        niche_start: u128,
    },
}
