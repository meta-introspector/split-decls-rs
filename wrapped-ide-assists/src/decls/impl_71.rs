macro_rules! deps {
    () => {
        ReferenceConversionType!();
        ReferenceConversion!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'db > ReferenceConversion < 'db > { pub (crate) fn convert_type (& self , db : & 'db dyn HirDatabase , display_target : DisplayTarget ,) -> ast :: Type { let ty = match self . conversion { ReferenceConversionType :: Copy => self . ty . display (db , display_target) . to_string () , ReferenceConversionType :: AsRefStr => "&str" . to_owned () , ReferenceConversionType :: AsRefSlice => { let type_argument_name = self . ty . type_arguments () . next () . unwrap () . display (db , display_target) . to_string () ; format ! ("&[{type_argument_name}]") } ReferenceConversionType :: Dereferenced => { let type_argument_name = self . ty . type_arguments () . next () . unwrap () . display (db , display_target) . to_string () ; format ! ("&{type_argument_name}") } ReferenceConversionType :: Option => { let type_argument_name = self . ty . type_arguments () . next () . unwrap () . display (db , display_target) . to_string () ; format ! ("Option<&{type_argument_name}>") } ReferenceConversionType :: Result => { let mut type_arguments = self . ty . type_arguments () ; let first_type_argument_name = type_arguments . next () . unwrap () . display (db , display_target) . to_string () ; let second_type_argument_name = type_arguments . next () . unwrap () . display (db , display_target) . to_string () ; format ! ("Result<&{first_type_argument_name}, &{second_type_argument_name}>") } } ; make :: ty (& ty) } pub (crate) fn getter (& self , field_name : String) -> ast :: Expr { let expr = make :: expr_field (make :: ext :: expr_self () , & field_name) ; match self . conversion { ReferenceConversionType :: Copy => expr , ReferenceConversionType :: AsRefStr | ReferenceConversionType :: AsRefSlice | ReferenceConversionType :: Dereferenced | ReferenceConversionType :: Option | ReferenceConversionType :: Result => { if self . impls_deref { make :: expr_ref (expr , false) } else { make :: expr_method_call (expr , make :: name_ref ("as_ref") , make :: arg_list ([])) . into () } } } } }
    };
}

impl_71!();