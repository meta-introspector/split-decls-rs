// Generated macro for check (function)
macro_rules! Depcrate_derive_unsafe_derive_deserializecheck {
() => {
// Module: crate::derive::unsafe_derive_deserialize
// Provides: {"check"}
// Dependencies: {}
# [doc = " Implementation of the `UNSAFE_DERIVE_DESERIALIZE` lint."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < '_ > , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx > , adt_hir_id : HirId ,) { fn has_unsafe < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) -> bool { let mut visitor = UnsafeVisitor { cx } ; walk_item (& mut visitor , item) . is_break () } if let Some (trait_def_id) = trait_ref . trait_def_id () && paths :: SERDE_DESERIALIZE . matches (cx , trait_def_id) && let ty :: Adt (def , _) = ty . kind () && ! is_lint_allowed (cx , UNSAFE_DERIVE_DESERIALIZE , adt_hir_id) && cx . tcx . inherent_impls (def . did ()) . iter () . map (| imp_did | cx . tcx . hir_expect_item (imp_did . expect_local ())) . any (| imp | has_unsafe (cx , imp)) { span_lint_hir_and_then (cx , UNSAFE_DERIVE_DESERIALIZE , adt_hir_id , item . span , "you are deriving `serde::Deserialize` on a type that has methods using `unsafe`" , | diag | { diag . help ("consider implementing `serde::Deserialize` manually. See https://serde.rs/impl-deserialize.html" ,) ; } ,) ; } }
};
}
