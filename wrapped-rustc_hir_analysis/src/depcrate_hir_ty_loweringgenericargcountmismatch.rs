// Generated macro for GenericArgCountMismatch (struct)
macro_rules! Depcrate_hir_ty_loweringGenericArgCountMismatch {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"GenericArgCountMismatch"}
// Dependencies: {}
# [doc = " A marker denoting that the generic arguments that were"] # [doc = " provided did not match the respective generic parameters."] # [derive (Clone , Debug)] pub struct GenericArgCountMismatch { pub reported : ErrorGuaranteed , # [doc = " A list of indices of arguments provided that were not valid."] pub invalid_args : Vec < usize > , }
};
}
