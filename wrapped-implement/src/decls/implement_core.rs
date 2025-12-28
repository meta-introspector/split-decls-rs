macro_rules! deps {
    () => {
        ImplementAttributes!();
        ImplementInputs!();
    };
}

macro_rules! implement_core {
    () => {
        deps!();
        fn implement_core (attributes : proc_macro2 :: TokenStream , item_tokens : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let attributes = syn :: parse2 :: < ImplementAttributes > (attributes) . unwrap () ; let original_type = syn :: parse2 :: < syn :: ItemStruct > (item_tokens) . unwrap () ; let inputs = ImplementInputs { original_ident : original_type . ident . clone () , interface_chains : convert_implements_to_interface_chains (attributes . implement) , trust_level : attributes . trust_level , agile : attributes . agile , impl_ident : quote :: format_ident ! ("{}_Impl" , & original_type . ident) , constraints : { if let Some (where_clause) = & original_type . generics . where_clause { where_clause . predicates . to_token_stream () } else { quote ! () } } , generics : if ! original_type . generics . params . is_empty () { let mut params = quote ! { } ; original_type . generics . params . to_tokens (& mut params) ; quote ! { <# params > } } else { quote ! { <> } } , is_generic : ! original_type . generics . params . is_empty () , original_type , } ; let items = gen_all (& inputs) ; let mut tokens = inputs . original_type . into_token_stream () ; for item in items { tokens . extend (item . into_token_stream ()) ; } tokens }
    };
}

implement_core!()