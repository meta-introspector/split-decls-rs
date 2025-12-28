macro_rules! deps {
    () => {
        Attr!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Parse for Attr { fn parse (input : ParseStream) -> parse :: Result < Self > { let lookahead = input . lookahead1 () ; if lookahead . peek (Token ! [type]) { input . parse () . map (Attr :: Type) } else if lookahead . peek (Ident) { let ident : Ident = input . parse () ? ; if ident == "target" { let _eq : Token ! [=] = input . parse () ? ; let target : Ident = input . parse () ? ; Ok (Attr :: Target (target)) } else { Err (Error :: new (ident . span () , "expected 'target'")) } } else { Err (lookahead . error ()) } } }
    };
}

impl_1!()