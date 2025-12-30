// Generated macro for Variant (struct)
macro_rules! Depcrate_hirVariant {
() => {
// Module: crate::hir
// Provides: {"Variant"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Variant < 'hir > { # [doc = " Name of the variant."] pub ident : Ident , # [doc = " Id of the variant (not the constructor, see `VariantData::ctor_hir_id()`)."] # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , # [doc = " Fields and constructor id of the variant."] pub data : VariantData < 'hir > , # [doc = " Explicit discriminant (e.g., `Foo = 1`)."] pub disr_expr : Option < & 'hir AnonConst > , # [doc = " Span"] pub span : Span , }
};
}
