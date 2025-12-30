// Generated macro for variable_kinds_from_iter (function)
macro_rules! Depcratevariable_kinds_from_iter {
() => {
// Module: crate
// Provides: {"variable_kinds_from_iter"}
// Dependencies: {}
pub (crate) fn variable_kinds_from_iter (db : & dyn HirDatabase , iter : impl Iterator < Item = hir_def :: GenericParamId > ,) -> VariableKinds { VariableKinds :: from_iter (Interner , iter . map (| x | match x { hir_def :: GenericParamId :: ConstParamId (id) => { chalk_ir :: VariableKind :: Const (db . const_param_ty (id)) } hir_def :: GenericParamId :: TypeParamId (_) => { chalk_ir :: VariableKind :: Ty (chalk_ir :: TyVariableKind :: General) } hir_def :: GenericParamId :: LifetimeParamId (_) => chalk_ir :: VariableKind :: Lifetime , }) ,) }
};
}
