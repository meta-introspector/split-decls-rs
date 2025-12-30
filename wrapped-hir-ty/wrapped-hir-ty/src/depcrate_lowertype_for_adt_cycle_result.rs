// Generated macro for type_for_adt_cycle_result (function)
macro_rules! Depcrate_lowertype_for_adt_cycle_result {
() => {
// Module: crate::lower
// Provides: {"type_for_adt_cycle_result"}
// Dependencies: {}
fn type_for_adt_cycle_result (db : & dyn HirDatabase , adt : AdtId) -> Binders < Ty > { let generics = generics (db , adt . into ()) ; make_binders (db , & generics , TyKind :: Error . intern (Interner)) }
};
}
