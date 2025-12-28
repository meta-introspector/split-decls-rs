macro_rules! test_fun_to_string {
    () => {
        # [test] fn test_fun_to_string () { create_default_session_globals_then (| | { let abba_ident = Ident :: from_str ("abba") ; let decl = ast :: FnDecl { inputs : ThinVec :: new () , output : ast :: FnRetTy :: Default (DUMMY_SP) } ; let generics = ast :: Generics :: default () ; assert_eq ! (fun_to_string (& decl , ast :: FnHeader :: default () , abba_ident , & generics) , "fn abba()") ; }) }
    };
}

test_fun_to_string!();