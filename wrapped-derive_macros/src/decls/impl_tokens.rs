macro_rules! impl_tokens {
    () => {
        fn impl_tokens (impl_generics : syn :: ImplGenerics , ident : & syn :: Ident , ty_generics : syn :: TypeGenerics , bounded_where_clause : & syn :: WhereClause , stream_body : proc_macro2 :: TokenStream , tag_body : Option < proc_macro2 :: TokenStream > ,) -> proc_macro2 :: TokenStream { let stream_fn = quote ! (fn stream <'sval , __SvalStream : sval :: Stream <'sval > + ? Sized > (&'sval self , stream : & mut __SvalStream) -> sval :: Result { # stream_body }) ; let tag_fn = if let Some (tag_body) = tag_body { quote ! (fn tag (& self) -> Option < sval :: Tag > { # tag_body }) } else { quote ! () } ; quote ! { const _ : () = { extern crate sval ; impl # impl_generics sval :: Value for # ident # ty_generics # bounded_where_clause { # stream_fn # tag_fn } } ; } }
    };
}

impl_tokens!();