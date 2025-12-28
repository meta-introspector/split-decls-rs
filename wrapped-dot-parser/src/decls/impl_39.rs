macro_rules! deps {
    () => {
        AttrList!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [cfg (feature = "to_tokens")] impl ToTokens for AttrList < (String , String) > { fn to_tokens (& self , ts : & mut proc_macro2 :: TokenStream) { let elems = & self . elems ; let tokens = quote ! { dot_parser :: ast :: AttrList { elems : std :: vec ! [# (# elems) ,*] } } ; ts . append_all (tokens) ; } }
    };
}

impl_39!();