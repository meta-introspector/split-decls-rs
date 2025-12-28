macro_rules! deps {
    () => {
        Param!();
        TokenStream!();
    };
}

macro_rules! write_invoke_arg {
    () => {
        deps!();
        fn write_invoke_arg (param : & Param) -> TokenStream { let name = param . write_ident () ; if param . is_input () && param . is_interface () { quote ! { core :: mem :: transmute_copy (&# name) } } else if (! param . is_pointer () && param . is_interface ()) || (param . is_input () && ! param . is_primitive ()) { quote ! { core :: mem :: transmute (&# name) } } else { quote ! { core :: mem :: transmute_copy (&# name) } } }
    };
}

write_invoke_arg!();