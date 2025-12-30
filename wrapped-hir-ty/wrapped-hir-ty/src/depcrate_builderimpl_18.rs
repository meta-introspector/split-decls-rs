// Generated macro for impl_18 (impl)
macro_rules! Depcrate_builderimpl_18 {
() => {
// Module: crate::builder
// Provides: {"impl_18"}
// Dependencies: {}
impl TyBuilder < hir_def :: AdtId > { pub fn adt (db : & dyn HirDatabase , def : hir_def :: AdtId) -> TyBuilder < hir_def :: AdtId > { TyBuilder :: subst_for_def (db , def , None) . with_data (def) } pub fn fill_with_defaults (mut self , db : & dyn HirDatabase , mut fallback : impl FnMut () -> Ty ,) -> Self { let defaults = db . generic_defaults (self . data . into ()) ; if let Some (defaults) = defaults . get (self . vec . len () ..) { for default_ty in defaults { if let Some (x) = default_ty . skip_binders () . ty (Interner) && x . is_unknown () { self . vec . push (fallback () . cast (Interner)) ; continue ; } self . vec . push (default_ty . clone () . substitute (Interner , & * self . vec) . cast (Interner)) ; } } let filler = self . param_kinds [self . vec . len () ..] . iter () . map (| x | match x { ParamKind :: Type => fallback () . cast (Interner) , ParamKind :: Const (ty) => unknown_const_as_generic (ty . clone ()) , ParamKind :: Lifetime => error_lifetime () . cast (Interner) , }) ; self . vec . extend (filler . casted (Interner)) ; self } pub fn build (self) -> Ty { let (adt , subst) = self . build_internal () ; TyKind :: Adt (AdtId (adt) , subst) . intern (Interner) } }
};
}
