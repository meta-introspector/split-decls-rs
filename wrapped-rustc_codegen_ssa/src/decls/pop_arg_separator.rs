macro_rules! pop_arg_separator {
    () => {
        fn pop_arg_separator (output : & mut String) { if output . ends_with (' ') { output . pop () ; } assert ! (output . ends_with (',')) ; output . pop () ; }
    };
}

pop_arg_separator!()