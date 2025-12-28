macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { ParseErrorKind :: InvalidNamedFlag { got } => { let _got = got ; write ! (f , "unrecognized named flag") ? ; # [cfg (feature = "std")] { write ! (f , " `{}`" , _got) ? ; } } ParseErrorKind :: InvalidHexFlag { got } => { let _got = got ; write ! (f , "invalid hex flag") ? ; # [cfg (feature = "std")] { write ! (f , " `{}`" , _got) ? ; } } ParseErrorKind :: EmptyFlag => { write ! (f , "encountered empty flag") ? ; } } Ok (()) } }
    };
}

impl_25!()