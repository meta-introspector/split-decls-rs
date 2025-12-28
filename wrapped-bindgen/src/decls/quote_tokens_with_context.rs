macro_rules! quote_tokens_with_context {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! quote_tokens_with_context { ($ tokens : ident ($ ($ b3 : tt) *) ($ ($ b2 : tt) *) ($ ($ b1 : tt) *) ($ ($ curr : tt) *) ($ ($ a1 : tt) *) ($ ($ a2 : tt) *) ($ ($ a3 : tt) *)) => { $ ($ crate :: quote_token_with_context ! ($ tokens $ b3 $ b2 $ b1 $ curr $ a1 $ a2 $ a3) ;) * } ; }
    };
}

quote_tokens_with_context!()