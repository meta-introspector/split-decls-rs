macro_rules! deps {
    () => {
        Conversion!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Conversion { fn conversion_expr (& self , i : & Ident) -> Expr { match * self { Conversion :: Into => parse_quote ! (# i . into ()) , Conversion :: AsRef => parse_quote ! (# i . as_ref ()) , Conversion :: AsMut => parse_quote ! (# i . as_mut ()) , } } }
    };
}

impl_3!()