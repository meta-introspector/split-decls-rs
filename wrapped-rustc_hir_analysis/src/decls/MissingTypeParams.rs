macro_rules! MissingTypeParams {
    () => {
        pub (crate) struct MissingTypeParams { pub span : Span , pub def_span : Span , pub span_snippet : Option < String > , pub missing_type_params : Vec < Symbol > , pub empty_generic_args : bool , }
    };
}

MissingTypeParams!()