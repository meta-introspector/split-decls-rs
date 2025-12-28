macro_rules! test_variant_to_string {
    () => {
        # [test] fn test_variant_to_string () { create_default_session_globals_then (| | { let ident = Ident :: from_str ("principal_skinner") ; let var = ast :: Variant { ident , vis : ast :: Visibility { span : DUMMY_SP , kind : ast :: VisibilityKind :: Inherited , tokens : None , } , attrs : ast :: AttrVec :: new () , id : ast :: DUMMY_NODE_ID , data : ast :: VariantData :: Unit (ast :: DUMMY_NODE_ID) , disr_expr : None , span : DUMMY_SP , is_placeholder : false , } ; let varstr = variant_to_string (& var) ; assert_eq ! (varstr , "principal_skinner") ; }) }
    };
}

test_variant_to_string!()