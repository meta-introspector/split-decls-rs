// Generated macro for ConstBlock (struct)
macro_rules! Depcrate_hirConstBlock {
() => {
// Module: crate::hir
// Provides: {"ConstBlock"}
// Dependencies: {}
# [doc = " An inline constant expression `const { something }`."] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct ConstBlock { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub body : BodyId , }
};
}
