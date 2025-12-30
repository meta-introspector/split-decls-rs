// Generated macro for Params (struct)
macro_rules! Depcrate_only_used_in_recursionParams {
() => {
// Module: crate::only_used_in_recursion
// Provides: {"Params"}
// Dependencies: {}
# [doc = " The parameters being checked by the lint, indexed by both the parameter's `HirId` and the"] # [doc = " `DefId` of the function paired with the parameter's index."] # [derive (Default)] # [expect (clippy :: struct_field_names)] struct Params { params : Vec < Param > , by_id : HirIdMap < usize > , by_fn : FxHashMap < (DefId , usize) , usize > , }
};
}
