macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for AList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let elems = & self . elems ; let elems = elems . iter () . map (| (s1 , s2) | quote ! { (# s1 , # s2) }) ; let tokens = quote ! { dot_parser :: ast :: AList { elems : std :: vec ! [# (# elems) ,*] } } ; ts . append_all (tokens) ; } }
    };
}

impl_46!()