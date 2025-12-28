macro_rules! deps {
    () => {
        CfgExpr!();
        ParseError!();
        Parser!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl FromStr for CfgExpr { type Err = ParseError ; fn from_str (s : & str) -> Result < CfgExpr , Self :: Err > { let mut p = Parser :: new (s) ; let e = p . expr () ? ; if let Some (rest) = p . rest () { return Err (ParseError :: new (p . t . orig , UnterminatedExpression (rest . to_string ()) ,)) ; } Ok (e) } }
    };
}

impl_16!()