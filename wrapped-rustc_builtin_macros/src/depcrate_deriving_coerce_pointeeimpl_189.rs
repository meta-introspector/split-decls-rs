// Generated macro for impl_189 (impl)
macro_rules! Depcrate_deriving_coerce_pointeeimpl_189 {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"impl_189"}
// Dependencies: {}
impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonGenericPointeeAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } fn visit_generic_param (& mut self , param : & 'a rustc_ast :: GenericParam) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; match & param . kind { GenericParamKind :: Type { default } => { rustc_ast :: visit :: visit_opt ! (error_on_pointee , visit_ty , default) ; } GenericParamKind :: Const { .. } | GenericParamKind :: Lifetime => { rustc_ast :: visit :: walk_generic_param (& mut error_on_pointee , param) ; } } } fn visit_ty (& mut self , t : & 'a rustc_ast :: Ty) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; error_on_pointee . visit_ty (t) } }
};
}
