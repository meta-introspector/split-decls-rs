macro_rules! deps {
    () => {
        Validators!();
    };
}

macro_rules! FieldResolverParameter {
    () => {
        deps!();
        struct FieldResolverParameter < 'a > { ty : & 'a Type , process_with : & 'a Option < Expr > , validator : & 'a Option < Validators > , ident : PatIdent , name : String , default : Option < proc_macro2 :: TokenStream > , }
    };
}

FieldResolverParameter!()