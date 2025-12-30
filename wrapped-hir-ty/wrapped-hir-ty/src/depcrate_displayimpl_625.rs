// Generated macro for impl_625 (impl)
macro_rules! Depcrate_displayimpl_625 {
() => {
// Module: crate::display
// Provides: {"impl_625"}
// Dependencies: {}
impl HirDisplay for WhereClause { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { if f . should_truncate () { return write ! (f , "{TYPE_HINT_TRUNCATION}") ; } match self { WhereClause :: Implemented (trait_ref) => { trait_ref . self_type_parameter (Interner) . hir_fmt (f) ? ; write ! (f , ": ") ? ; trait_ref . hir_fmt (f) ? ; } WhereClause :: AliasEq (AliasEq { alias : AliasTy :: Projection (projection_ty) , ty }) => { write ! (f , "<") ? ; let trait_ref = & projection_ty . trait_ref (f . db) ; trait_ref . self_type_parameter (Interner) . hir_fmt (f) ? ; write ! (f , " as ") ? ; trait_ref . hir_fmt (f) ? ; write ! (f , ">::" ,) ? ; let type_alias = from_assoc_type_id (projection_ty . associated_ty_id) ; f . start_location_link (type_alias . into ()) ; write ! (f , "{}" , f . db . type_alias_signature (type_alias) . name . display (f . db , f . edition ()) ,) ? ; f . end_location_link () ; write ! (f , " = ") ? ; ty . hir_fmt (f) ? ; } WhereClause :: AliasEq (_) => write ! (f , "{{error}}") ? , WhereClause :: TypeOutlives (..) => { } WhereClause :: LifetimeOutlives (..) => { } } Ok (()) } }
};
}
