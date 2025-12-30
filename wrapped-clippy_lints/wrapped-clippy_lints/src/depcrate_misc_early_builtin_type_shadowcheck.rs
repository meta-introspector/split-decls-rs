// Generated macro for check (function)
macro_rules! Depcrate_misc_early_builtin_type_shadowcheck {
() => {
// Module: crate::misc_early::builtin_type_shadow
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , param : & GenericParam) { if let GenericParamKind :: Type { .. } = param . kind && let Some (prim_ty) = PrimTy :: from_name (param . ident . name) { span_lint (cx , BUILTIN_TYPE_SHADOW , param . ident . span , format ! ("this generic shadows the built-in type `{}`" , prim_ty . name ()) ,) ; } }
};
}
