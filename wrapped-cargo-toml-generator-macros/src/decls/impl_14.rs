macro_rules! deps {
    () => {
        KeyValue!();
        InlineTable!();
        RootItem!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Parse for RootItem { fn parse (input : ParseStream) -> Result < Self > { let id : Ident = input . parse () ? ; let next_lookahead = input . lookahead1 () ; if next_lookahead . peek (Brace) { let content ; braced ! (content in input) ; Ok (RootItem :: Section (id , content . parse :: < proc_macro2 :: TokenStream > () ? ,)) } else if next_lookahead . peek (Token ! [=]) { input . parse :: < Token ! [=] > () ? ; let lookahead_val = input . lookahead1 () ; if lookahead_val . peek (LitStr) { let val : LitStr = input . parse () ? ; Ok (RootItem :: KeyValue (KeyValue :: Simple (id , val))) } else if lookahead_val . peek (Brace) { let content ; braced ! (content in input) ; Ok (RootItem :: KeyValue (KeyValue :: InlineTable (id , content . parse :: < proc_macro2 :: TokenStream > () ? ,))) } else if lookahead_val . peek (Bracket) { let content ; bracketed ! (content in input) ; Ok (RootItem :: KeyValue (KeyValue :: List (id , content . parse :: < proc_macro2 :: TokenStream > () ? ,))) } else { Err (input . error ("expected string, braced block, or bracketed list after '='")) } } else { Err (input . error ("expected '{' for section or '=' for key-value pair")) } } }
    };
}

impl_14!();