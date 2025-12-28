macro_rules! return_impl_trait {
    () => {
        macro_rules ! return_impl_trait { ($ (# [$ attr : meta]) * $ vis : vis fn $ name : ident <$ param : ident > $ args : tt -> $ impl_trait : ty [$ concrete : ty] $ ($ body : tt) +) => { # [cfg (not (docsrs))] $ (# [$ attr]) * $ vis fn $ name <$ param > $ args -> $ concrete $ ($ body) + # [cfg (docsrs)] $ (# [$ attr]) * $ vis fn $ name <$ param > $ args -> $ impl_trait $ ($ body) + } ; }
    };
}

return_impl_trait!()