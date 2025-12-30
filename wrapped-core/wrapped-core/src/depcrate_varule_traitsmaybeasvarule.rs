// Generated macro for MaybeAsVarULE (trait)
macro_rules! Depcrate_varule_traitsMaybeAsVarULE {
() => {
// Module: crate::varule_traits
// Provides: {"MaybeAsVarULE"}
// Dependencies: {}
# [doc = " A trait that associates a [`VarULE`] type with a data struct."] # [doc = ""] # [doc = " Some data structs can be represented compactly as a single [`VarULE`],"] # [doc = " such as `str` or a packed pattern. This trait allows for data providers"] # [doc = " to use optimizations for such types."] # [doc = ""] # [doc = " ❗ Not all data structs benefit from this optimization. It works best when the"] # [doc = " data struct is multiplied across a large number of data marker attributes."] # [doc = ""] # [doc = " Both [`MaybeAsVarULE`] and [`MaybeEncodeAsVarULE`] should be implemented"] # [doc = " on all data structs. The [`data_struct!`](crate::data_struct) macro provides an impl."] pub trait MaybeAsVarULE { # [doc = " The [`VarULE`] type for this data struct, or `[()]`"] # [doc = " if it cannot be represented as [`VarULE`]."] type EncodedStruct : ? Sized + VarULE ; }
};
}
