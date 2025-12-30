// Generated macro for check_unsafe_derive_deserialize (function)
macro_rules! Depcrate_derivecheck_unsafe_derive_deserialize {
() => {
// Module: crate::derive
// Provides: {"check_unsafe_derive_deserialize"}
// Dependencies: {}
# [doc = " Implementation of the `UNSAFE_DERIVE_DESERIALIZE` lint."] fn check_unsafe_derive_deserialize < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < '_ > , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx > ,) { fn has_unsafe < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) -> bool { let mut visitor = UnsafeVisitor { cx } ; walk_item (& mut visitor , item) . is_break () } if let Some (trait_def_id) = trait_ref . trait_def_id () && paths :: SERDE_DESERIALIZE . matches (cx , trait_def_id) && let ty :: Adt (def , _) = ty . kind () && let Some (local_def_id) = def . did () . as_local () && let adt_hir_id = cx . tcx . local_def_id_to_hir_id (local_def_id) && ! is_lint_allowed (cx , UNSAFE_DERIVE_DESERIALIZE , adt_hir_id) && cx . tcx . inherent_impls (def . did ()) . iter () . map (| imp_did | cx . tcx . hir_expect_item (imp_did . expect_local ())) . any (| imp | has_unsafe (cx , imp)) { span_lint_hir_and_then (cx , UNSAFE_DERIVE_DESERIALIZE , adt_hir_id , item . span , "you are deriving `serde::Deserialize` on a type that has methods using `unsafe`" , | diag | { diag . help ("consider implementing `serde::Deserialize` manually. See https://serde.rs/impl-deserialize.html" ,) ; } ,) ; } }
};
}
