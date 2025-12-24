use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ReprOptions {
    #[inline]
    pub fn simd(&self) -> bool {
        self.flags.contains(ReprFlags::IS_SIMD)
    }
    #[inline]
    pub fn c(&self) -> bool {
        self.flags.contains(ReprFlags::IS_C)
    }
    #[inline]
    pub fn packed(&self) -> bool {
        self.pack.is_some()
    }
    #[inline]
    pub fn transparent(&self) -> bool {
        self.flags.contains(ReprFlags::IS_TRANSPARENT)
    }
    #[inline]
    pub fn linear(&self) -> bool {
        self.flags.contains(ReprFlags::IS_LINEAR)
    }
    /// Returns the discriminant type, given these `repr` options.
    /// This must only be called on enums!
    ///
    /// This is the "typeck type" of the discriminant, which is effectively the maximum size:
    /// discriminant values will be wrapped to fit (with a lint). Layout can later decide to use a
    /// smaller type for the tag that stores the discriminant at runtime and that will work just
    /// fine, it just induces casts when getting/setting the discriminant.
    pub fn discr_type(&self) -> IntegerType {
        self.int.unwrap_or(IntegerType::Pointer(true))
    }
    /// Returns `true` if this `#[repr()]` should inhabit "smart enum
    /// layout" optimizations, such as representing `Foo<&T>` as a
    /// single pointer.
    pub fn inhibit_enum_layout_opt(&self) -> bool {
        self.c() || self.int.is_some()
    }
    pub fn inhibit_newtype_abi_optimization(&self) -> bool {
        self.flags.intersects(ReprFlags::ABI_UNOPTIMIZABLE)
    }
    /// Returns `true` if this `#[repr()]` guarantees a fixed field order,
    /// e.g. `repr(C)` or `repr(<int>)`.
    pub fn inhibit_struct_field_reordering(&self) -> bool {
        self.flags.intersects(ReprFlags::FIELD_ORDER_UNOPTIMIZABLE) || self.int.is_some()
    }
    /// Returns `true` if this type is valid for reordering and `-Z randomize-layout`
    /// was enabled for its declaration crate.
    pub fn can_randomize_type_layout(&self) -> bool {
        !self.inhibit_struct_field_reordering()
            && self.flags.contains(ReprFlags::RANDOMIZE_LAYOUT)
    }
    /// Returns `true` if this `#[repr()]` should inhibit union ABI optimisations.
    pub fn inhibits_union_abi_opt(&self) -> bool {
        self.c()
    }
}
