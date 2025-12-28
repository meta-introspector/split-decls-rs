macro_rules! deps {
    () => {
        ImplementAttributes!();
        UseTree2!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl ImplementAttributes { fn parse_implement (& mut self , cursor : syn :: parse :: ParseStream) -> syn :: parse :: Result < () > { let tree = cursor . parse :: < UseTree2 > () ? ; self . walk_implement (& tree , & mut String :: new ()) ? ; if ! cursor . is_empty () { cursor . parse :: < syn :: Token ! [,] > () ? ; } Ok (()) } fn walk_implement (& mut self , tree : & UseTree2 , namespace : & mut String ,) -> syn :: parse :: Result < () > { match tree { UseTree2 :: Path (input) => { if ! namespace . is_empty () { namespace . push_str ("::") ; } namespace . push_str (& input . ident . to_string ()) ; self . walk_implement (& input . tree , namespace) ? ; } UseTree2 :: Name (_) => { self . implement . push (tree . to_element_type (namespace) ?) ; } UseTree2 :: Group (input) => { for tree in & input . items { self . walk_implement (tree , namespace) ? ; } } UseTree2 :: TrustLevel (input) => self . trust_level = * input , UseTree2 :: Agile (agile) => self . agile = * agile , } Ok (()) } }
    };
}

impl_18!();