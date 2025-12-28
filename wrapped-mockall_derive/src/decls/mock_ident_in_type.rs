macro_rules! mock_ident_in_type {
    () => {
        # [doc = " Add \"Mock\" to the front of the named type"] fn mock_ident_in_type (ty : & mut Type) { match ty { Type :: Path (type_path) => { if type_path . path . segments . len () != 1 { compile_error (type_path . path . span () , "mockall_derive only supports structs defined in the current module") ; return ; } let ident = & mut type_path . path . segments . last_mut () . unwrap () . ident ; * ident = gen_mock_ident (ident) } , x => { compile_error (x . span () , "mockall_derive only supports mocking traits and structs") ; } } ; }
    };
}

mock_ident_in_type!();