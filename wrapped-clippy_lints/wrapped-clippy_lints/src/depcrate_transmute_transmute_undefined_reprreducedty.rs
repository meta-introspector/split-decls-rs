// Generated macro for ReducedTy (enum)
macro_rules! Depcrate_transmute_transmute_undefined_reprReducedTy {
() => {
// Module: crate::transmute::transmute_undefined_repr
// Provides: {"ReducedTy"}
// Dependencies: {}
enum ReducedTy < 'tcx > { # [doc = " The type can be used for type erasure."] TypeErasure { raw_ptr_only : bool } , # [doc = " The type is a struct containing either zero non-zero sized fields, or multiple non-zero"] # [doc = " sized fields with a defined order."] # [doc = " The value is the first non-zero sized type."] OrderedFields (Option < Ty < 'tcx > >) , # [doc = " The type is a struct containing multiple non-zero sized fields with no defined order."] UnorderedFields (Ty < 'tcx >) , # [doc = " Any other type."] Other (Ty < 'tcx >) , }
};
}
