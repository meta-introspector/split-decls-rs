macro_rules! deps {
    () => {
        MockItemModule!();
        MockItemContent!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl ToTokens for MockItemModule { fn to_tokens (& self , tokens : & mut TokenStream) { let mut body = TokenStream :: new () ; let mut cp_body = TokenStream :: new () ; let attrs = & self . attrs ; let modname = & self . mock_ident ; let vis = & self . vis ; for item in self . content . iter () { match item { MockItemContent :: Tokens (ts) => ts . to_tokens (& mut body) , MockItemContent :: Fn (f) => { let call = f . call (None) ; let ctx_fn = f . context_fn (None) ; let priv_mod = f . priv_module () ; quote ! (# priv_mod # call # ctx_fn) . to_tokens (& mut body) ; f . checkpoint () . to_tokens (& mut cp_body) ; } , } } quote ! (# [doc = " Verify that all current expectations for every function in"] # [doc = " this module are satisfied and clear them."] pub fn checkpoint () { # cp_body }) . to_tokens (& mut body) ; let docstr = { if let Some (ident) = & self . orig_ident { let inner = format ! ("Mock version of the `{ident}` module") ; quote ! (# [doc = # inner]) } else { quote ! (# [allow (missing_docs)]) } } ; quote ! (# [allow (unused_imports)] # attrs # docstr # vis mod # modname { # body }) . to_tokens (tokens) ; } }
    };
}

impl_60!();