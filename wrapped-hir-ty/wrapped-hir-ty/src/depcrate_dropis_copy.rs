// Generated macro for is_copy (function)
macro_rules! Depcrate_dropis_copy {
() => {
// Module: crate::drop
// Provides: {"is_copy"}
// Dependencies: {}
fn is_copy (db : & dyn HirDatabase , ty : Ty , env : Arc < TraitEnvironment >) -> bool { let Some (copy_trait) = LangItem :: Copy . resolve_trait (db , env . krate) else { return false ; } ; let trait_ref = TyBuilder :: trait_ref (db , copy_trait) . push (ty) . build () ; let goal = Canonical { value : InEnvironment :: new (& env . env , trait_ref . cast (Interner)) , binders : CanonicalVarKinds :: empty (Interner) , } ; db . trait_solve (env . krate , env . block , goal) . is_some () }
};
}
