macro_rules! deps {
    () => {
        TypeDirectiveLocation!();
        TypeDirective!();
        Directive!();
    };
}

macro_rules! gen_directive_calls {
    () => {
        deps!();
        pub fn gen_directive_calls (directive_calls : & [Expr] , location : TypeDirectiveLocation ,) -> Vec < TokenStream > { directive_calls . iter () . map (| directive | { let directive_path = extract_directive_call_path (directive) . expect ("Directive invocation expression format must be [<directive_path>::]<directive_name>::apply(<args>)" ,) ; let identifier = location . location_trait_identifier () ; quote ! ({ <# directive_path as async_graphql :: registry :: location_traits ::# identifier >:: check () ; <# directive_path as async_graphql :: TypeDirective >:: register (&# directive_path , registry) ; # directive }) }) . collect :: < Vec < _ > > () }
    };
}

gen_directive_calls!()