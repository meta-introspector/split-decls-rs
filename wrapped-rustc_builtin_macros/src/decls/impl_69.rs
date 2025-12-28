macro_rules! deps {
    () => {
        AlwaysErrorOnGenericParam!();
        NonGenericPointee!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for AlwaysErrorOnGenericParam < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } }
    };
}

impl_69!()