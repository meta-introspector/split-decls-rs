macro_rules! deps {
    () => {
        EarlyContext!();
        NonCamelCaseType!();
        NonCamelCaseTypeSub!();
    };
}

macro_rules! impl_696 {
    () => {
        deps!();
        impl NonCamelCaseTypes { fn check_case (& self , cx : & EarlyContext < '_ > , sort : & str , ident : & Ident) { let name = ident . name . as_str () ; if ! is_camel_case (name) { let cc = to_camel_case (name) ; let sub = if * name != cc { NonCamelCaseTypeSub :: Suggestion { span : ident . span , replace : cc } } else { NonCamelCaseTypeSub :: Label { span : ident . span } } ; cx . emit_span_lint (NON_CAMEL_CASE_TYPES , ident . span , NonCamelCaseType { sort , name , sub } ,) ; } } }
    };
}

impl_696!()