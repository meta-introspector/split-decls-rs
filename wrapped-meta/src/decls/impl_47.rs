macro_rules! deps {
    () => {
        ParserNode!();
        ParserExpr!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'i > ParserNode < 'i > { # [doc = " will remove nodes that do not match `f`"] pub fn filter_map_top_down < F , T > (self , mut f : F) -> Vec < T > where F : FnMut (ParserNode < 'i >) -> Option < T > , { pub fn filter_internal < 'i , F , T > (node : ParserNode < 'i > , f : & mut F , result : & mut Vec < T >) where F : FnMut (ParserNode < 'i >) -> Option < T > , { if let Some (value) = f (node . clone ()) { result . push (value) ; } match node . expr { ParserExpr :: PosPred (node) => { filter_internal (* node , f , result) ; } ParserExpr :: NegPred (node) => { filter_internal (* node , f , result) ; } ParserExpr :: Seq (lhs , rhs) => { filter_internal (* lhs , f , result) ; filter_internal (* rhs , f , result) ; } ParserExpr :: Choice (lhs , rhs) => { filter_internal (* lhs , f , result) ; filter_internal (* rhs , f , result) ; } ParserExpr :: Rep (node) => { filter_internal (* node , f , result) ; } ParserExpr :: RepOnce (node) => { filter_internal (* node , f , result) ; } ParserExpr :: RepExact (node , _) => { filter_internal (* node , f , result) ; } ParserExpr :: RepMin (node , _) => { filter_internal (* node , f , result) ; } ParserExpr :: RepMax (node , _) => { filter_internal (* node , f , result) ; } ParserExpr :: RepMinMax (node , ..) => { filter_internal (* node , f , result) ; } ParserExpr :: Opt (node) => { filter_internal (* node , f , result) ; } ParserExpr :: Push (node) => { filter_internal (* node , f , result) ; } _ => () , } } let mut result = vec ! [] ; filter_internal (self , & mut f , & mut result) ; result } }
    };
}

impl_47!();