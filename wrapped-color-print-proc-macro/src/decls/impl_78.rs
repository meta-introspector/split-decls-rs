macro_rules! deps {
    () => {
        FormatArg!();
        Result!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Parse for FormatArg { fn parse (input : ParseStream) -> Result < Self > { let arg_name : Option < (Ident , token :: Eq) > = if input . peek2 (Token ! [=]) { Some ((input . parse () ? , input . parse () ?)) } else { None } ; let expr : Expr = input . parse () ? ; Ok (FormatArg { arg_name , expr }) } }
    };
}

impl_78!()