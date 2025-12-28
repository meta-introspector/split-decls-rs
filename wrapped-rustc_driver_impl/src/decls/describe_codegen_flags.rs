macro_rules! describe_codegen_flags {
    () => {
        fn describe_codegen_flags () { safe_println ! ("\nAvailable codegen options:\n") ; print_flag_list ("-C" , config :: CG_OPTIONS) ; }
    };
}

describe_codegen_flags!();