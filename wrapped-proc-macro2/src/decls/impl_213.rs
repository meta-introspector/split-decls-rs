macro_rules! deps {
    () => {
        Group!();
        Ident!();
        Literal!();
        TokenTree!();
        Punct!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [doc = " Prints token tree in a form convenient for debugging."] impl Debug for TokenTree { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { TokenTree :: Group (t) => Debug :: fmt (t , f) , TokenTree :: Ident (t) => { let mut debug = f . debug_struct ("Ident") ; debug . field ("sym" , & format_args ! ("{}" , t)) ; imp :: debug_span_field_if_nontrivial (& mut debug , t . span () . inner) ; debug . finish () } TokenTree :: Punct (t) => Debug :: fmt (t , f) , TokenTree :: Literal (t) => Debug :: fmt (t , f) , } } }
    };
}

impl_213!();