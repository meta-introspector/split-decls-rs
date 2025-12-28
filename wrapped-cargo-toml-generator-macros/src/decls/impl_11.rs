macro_rules! deps {
    () => {
        KeyValue!();
        InlineTable!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Parse for KeyValue { fn parse (input : ParseStream) -> Result < Self > { let key : Ident = input . parse () ? ; let lookahead = input . lookahead1 () ; if lookahead . peek (Token ! [=]) { input . parse :: < Token ! [=] > () ? ; let lookahead_val = input . lookahead1 () ; if lookahead_val . peek (LitStr) { let val : LitStr = input . parse () ? ; Ok (KeyValue :: Simple (key , val)) } else if lookahead_val . peek (Brace) { let content ; braced ! (content in input) ; Ok (KeyValue :: InlineTable (key , content . parse :: < proc_macro2 :: TokenStream > () ? ,)) } else if lookahead_val . peek (Bracket) { let content ; bracketed ! (content in input) ; Ok (KeyValue :: List (key , content . parse :: < proc_macro2 :: TokenStream > () ? ,)) } else { Err (input . error ("expected string, braced block, or bracketed list after '='")) } } else if lookahead . peek (Brace) { let content ; braced ! (content in input) ; Ok (KeyValue :: Block (key , content . parse :: < proc_macro2 :: TokenStream > () ? ,)) } else { Err (input . error ("expected '=' or '{'")) } } }
    };
}

impl_11!();