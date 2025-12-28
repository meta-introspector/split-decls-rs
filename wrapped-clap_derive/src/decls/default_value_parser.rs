macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! default_value_parser {
    () => {
        deps!();
        fn default_value_parser (inner_type : & Type , span : Span) -> Method { let func = Ident :: new ("value_parser" , span) ; Method :: new (func , quote_spanned ! { span => clap :: value_parser ! (# inner_type) } ,) }
    };
}

default_value_parser!();