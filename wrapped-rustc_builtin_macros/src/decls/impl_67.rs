macro_rules! deps {
    () => {
        NonGenericPointee!();
        Ty!();
        DetectNonGenericPointeeAttr!();
        AlwaysErrorOnGenericParam!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonGenericPointeeAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } fn visit_generic_param (& mut self , param : & 'a rustc_ast :: GenericParam) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; match & param . kind { GenericParamKind :: Type { default } => { rustc_ast :: visit :: visit_opt ! (error_on_pointee , visit_ty , default) ; } GenericParamKind :: Const { .. } | GenericParamKind :: Lifetime => { rustc_ast :: visit :: walk_generic_param (& mut error_on_pointee , param) ; } } } fn visit_ty (& mut self , t : & 'a rustc_ast :: Ty) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; error_on_pointee . visit_ty (t) } }
    };
}

impl_67!()