macro_rules! phantom_default_inits {
    () => {
        fn phantom_default_inits (generics : & Generics) -> Vec < TokenStream > { generics . params . iter () . enumerate () . map (| (count , _param) | { let phident = format_ident ! ("_t{count}") ; quote ! (# phident : :: std :: marker :: PhantomData) }) . collect () }
    };
}

phantom_default_inits!()