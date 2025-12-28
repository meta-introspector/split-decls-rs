macro_rules! deps {
    () => {
        GeneratedTrait!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl ToTokens for GeneratedTrait { fn to_tokens (& self , tokens : & mut TokenStream) { let name = Ident :: new (& self . name , proc_macro2 :: Span :: call_site ()) ; let generics = & self . generics ; let where_clause = & self . where_clause ; let visibility = self . visibility . as_ref () . map_or_else (| | quote ! { } , | v | quote ! { # v }) ; let methods = self . methods . iter () . map (| m | { let sig : TokenStream = m . signature . parse () . expect ("Invalid method signature") ; quote ! { # sig } }) ; let associated_types = self . associated_types . iter () . map (| at | { let at_name = Ident :: new (& at . name , proc_macro2 :: Span :: call_site ()) ; let bounds = & at . bounds ; let default = & at . default ; quote ! { type # at_name : # (# bounds) * # default ; } }) ; let supertraits = & self . supertraits ; let supertraits_tokens = if supertraits . is_empty () { quote ! { } } else { quote ! { : # (# supertraits) ,* } } ; tokens . extend (quote ! { # visibility trait # name # generics # supertraits_tokens # where_clause { # (# associated_types) * # (# methods) * } }) ; } }
    };
}

impl_156!()