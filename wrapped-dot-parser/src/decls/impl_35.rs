macro_rules! deps {
    () => {
        Graph!();
        Node!();
        AttrStmt!();
        Edge!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for AttrStmt < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let new_tokens = match self { AttrStmt :: Graph (attrs) => quote ! { dot_parser :: ast :: AttrStmt :: Graph (# attrs) } , AttrStmt :: Node (attrs) => quote ! { dot_parser :: ast :: AttrStmt :: Node (# attrs) } , AttrStmt :: Edge (attrs) => quote ! { dot_parser :: ast :: AttrStmt :: Edge (# attrs) } , } ; ts . append_all (new_tokens) ; } }
    };
}

impl_35!()