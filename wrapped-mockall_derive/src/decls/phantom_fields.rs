macro_rules! phantom_fields {
    () => {
        # [doc = " Generate any PhantomData field definitions"] fn phantom_fields (generics : & Generics) -> Vec < TokenStream > { generics . params . iter () . enumerate () . filter_map (| (count , param) | { let phident = format_ident ! ("_t{count}") ; match param { syn :: GenericParam :: Lifetime (l) => { if ! l . bounds . is_empty () { compile_error (l . bounds . span () , "#automock does not yet support lifetime bounds on structs") ; } let lifetime = & l . lifetime ; Some (quote ! (# phident : :: std :: marker :: PhantomData <&# lifetime () >)) } , syn :: GenericParam :: Type (tp) => { let ty = & tp . ident ; Some (quote ! (# phident : :: std :: marker :: PhantomData <# ty >)) } , syn :: GenericParam :: Const (_) => { compile_error (param . span () , "#automock does not yet support generic constants") ; None } } }) . collect () }
    };
}

phantom_fields!();