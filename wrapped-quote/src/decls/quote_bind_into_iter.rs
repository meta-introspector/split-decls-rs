macro_rules! quote_bind_into_iter {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! quote_bind_into_iter { ($ has_iter : ident $ var : ident) => { # [allow (unused_mut)] let (mut $ var , i) = $ var . quote_into_iter () ; let $ has_iter = $ has_iter | i ; } ; }
    };
}

quote_bind_into_iter!()