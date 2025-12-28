macro_rules! deps {
    () => {
        HasIterator!();
        ThereIsNoIteratorInRepetition!();
        ToTokens!();
    };
}

macro_rules! quote_token_with_context {
    () => {
        deps!();
        # [macro_export] # [doc (hidden)] macro_rules ! quote_token_with_context { ($ tokens : ident $ b3 : tt $ b2 : tt $ b1 : tt @ $ a1 : tt $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt $ b2 : tt $ b1 : tt (#) ($ ($ inner : tt) *) * $ a3 : tt) => { { use $ crate :: tokens :: runtime :: ext ::*; let has_iter = $ crate :: tokens :: runtime :: ThereIsNoIteratorInRepetition ; $ crate :: pounded_var_names ! (quote_bind_into_iter ! (has_iter) () $ ($ inner) *) ; let _ : $ crate :: tokens :: runtime :: HasIterator = has_iter ; while true { $ crate :: pounded_var_names ! (quote_bind_next_or_break ! () () $ ($ inner) *) ; $ crate :: quote_each_token ! ($ tokens $ ($ inner) *) ; } } } ; ($ tokens : ident $ b3 : tt $ b2 : tt # (($ ($ inner : tt) *)) * $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt # ($ ($ inner : tt) *) (*) $ a1 : tt $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt $ b2 : tt $ b1 : tt (#) ($ ($ inner : tt) *) $ sep : tt *) => { { use $ crate :: tokens :: runtime :: ext ::*; let mut _i = 0usize ; let has_iter = $ crate :: tokens :: runtime :: ThereIsNoIteratorInRepetition ; $ crate :: pounded_var_names ! (quote_bind_into_iter ! (has_iter) () $ ($ inner) *) ; let _ : $ crate :: tokens :: runtime :: HasIterator = has_iter ; while true { $ crate :: pounded_var_names ! (quote_bind_next_or_break ! () () $ ($ inner) *) ; if _i > 0 { $ crate :: quote_token ! ($ tokens $ sep) ; } _i += 1 ; $ crate :: quote_each_token ! ($ tokens $ ($ inner) *) ; } } } ; ($ tokens : ident $ b3 : tt $ b2 : tt # (($ ($ inner : tt) *)) $ sep : tt * $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt # ($ ($ inner : tt) *) ($ sep : tt) * $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident # ($ ($ inner : tt) *) * (*) $ a1 : tt $ a2 : tt $ a3 : tt) => { $ crate :: quote_token ! ($ tokens *) ; } ; ($ tokens : ident # ($ ($ inner : tt) *) $ sep : tt (*) $ a1 : tt $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt $ b2 : tt $ b1 : tt (#) $ var : ident $ a2 : tt $ a3 : tt) => { $ crate :: tokens :: ToTokens :: to_tokens (&$ var , & mut $ tokens) ; } ; ($ tokens : ident $ b3 : tt $ b2 : tt # ($ var : ident) $ a1 : tt $ a2 : tt $ a3 : tt) => { } ; ($ tokens : ident $ b3 : tt $ b2 : tt $ b1 : tt ($ curr : tt) $ a1 : tt $ a2 : tt $ a3 : tt) => { $ crate :: quote_token ! ($ tokens $ curr) ; } ; }
    };
}

quote_token_with_context!();