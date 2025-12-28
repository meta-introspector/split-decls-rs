macro_rules! wrap_field_inits {
    () => {
        # [doc = " Given a set of entries for struct field definitions to go inside a `struct {}` definition,"] # [doc = " wrap in a () or {} based on the type of field"] pub fn wrap_field_inits (streams : & [TokenStream2] , fields : & Fields) -> TokenStream2 { match * fields { Fields :: Named (_) => quote ! ({ # (# streams) ,* }) , Fields :: Unnamed (_) => quote ! ((# (# streams) ,*)) , Fields :: Unit => { unreachable ! ("#[make_(var)ule] should have already checked that there are fields") } } }
    };
}

wrap_field_inits!()