use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub struct LayoutData<FieldIdx: Idx, VariantIdx: Idx> {
    /// Says where the fields are located within the layout.
    pub fields: FieldsShape<FieldIdx>,
    /// Encodes information about multi-variant layouts.
    /// Even with `Multiple` variants, a layout still has its own fields! Those are then
    /// shared between all variants. One of them will be the discriminant,
    /// but e.g. coroutines can have more.
    ///
    /// To access all fields of this layout, both `fields` and the fields of the active variant
    /// must be taken into account.
    pub variants: Variants<FieldIdx, VariantIdx>,
    /// The `backend_repr` defines how this data will be represented to the codegen backend,
    /// and encodes value restrictions via `valid_range`.
    ///
    /// Note that this is entirely orthogonal to the recursive structure defined by
    /// `variants` and `fields`; for example, `ManuallyDrop<Result<isize, isize>>` has
    /// `IrForm::ScalarPair`! So, even with non-`Memory` `backend_repr`, `fields` and `variants`
    /// have to be taken into account to find all fields of this layout.
    pub backend_repr: BackendRepr,
    /// The leaf scalar with the largest number of invalid values
    /// (i.e. outside of its `valid_range`), if it exists.
    pub largest_niche: Option<Niche>,
    /// Is this type known to be uninhabted?
    ///
    /// This is separate from BackendRepr because uninhabited return types can affect ABI,
    /// especially in the case of by-pointer struct returns, which allocate stack even when unused.
    pub uninhabited: bool,
    pub align: AbiAlign,
    pub size: Size,
    /// The largest alignment explicitly requested with `repr(align)` on this type or any field.
    /// Only used on i686-windows, where the argument passing ABI is different when alignment is
    /// requested, even if the requested alignment is equal to the natural alignment.
    pub max_repr_align: Option<Align>,
    /// The alignment the type would have, ignoring any `repr(align)` but including `repr(packed)`.
    /// Only used on aarch64-linux, where the argument passing ABI ignores the requested alignment
    /// in some cases.
    pub unadjusted_abi_align: Align,
    /// The randomization seed based on this type's own repr and its fields.
    ///
    /// Since randomization is toggled on a per-crate basis even crates that do not have randomization
    /// enabled should still calculate a seed so that downstream uses can use it to distinguish different
    /// types.
    ///
    /// For every T and U for which we do not guarantee that a repr(Rust) `Foo<T>` can be coerced or
    /// transmuted to `Foo<U>` we aim to create probalistically distinct seeds so that Foo can choose
    /// to reorder its fields based on that information. The current implementation is a conservative
    /// approximation of this goal.
    pub randomization_seed: Hash64,
}
