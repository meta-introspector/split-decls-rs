macro_rules! is_int_bool_float_or_char {
    () => {
        fn is_int_bool_float_or_char (ty : Ty < '_ >) -> bool { ty . is_bool () || ty . is_integral () || ty . is_char () || ty . is_floating_point () }
    };
}

is_int_bool_float_or_char!();