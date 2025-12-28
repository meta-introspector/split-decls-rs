macro_rules! deps {
    () => {
        ValueHint!();
        Arg!();
        Id!();
        ValueRange!();
    };
}

macro_rules! assert_arg {
    () => {
        deps!();
        fn assert_arg (arg : & Arg) { debug ! ("Arg::_debug_asserts:{}" , arg . get_id ()) ; assert ! (! arg . blacklist . iter () . any (| x | x == arg . get_id ()) , "Argument '{}' cannot conflict with itself" , arg . get_id () ,) ; assert ! (arg . get_num_args () . unwrap_or (1 . into ()) . max_values () <= arg . get_action () . max_num_args () . max_values () , "Argument `{}`'s action {:?} is incompatible with `num_args({:?})`" , arg . get_id () , arg . get_action () , arg . get_num_args () . unwrap_or (1 . into ())) ; if let Some (action_type_id) = arg . get_action () . value_type_id () { assert_eq ! (action_type_id , arg . get_value_parser () . type_id () , "Argument `{}`'s selected action {:?} contradicts `value_parser` ({:?})" , arg . get_id () , arg . get_action () , arg . get_value_parser ()) ; } if arg . get_value_hint () != ValueHint :: Unknown { assert ! (arg . is_takes_value_set () , "Argument '{}' has value hint but takes no value" , arg . get_id ()) ; if arg . get_value_hint () == ValueHint :: CommandWithArguments { assert ! (arg . is_multiple_values_set () , "Argument '{}' uses hint CommandWithArguments and must accept multiple values" , arg . get_id ()) ; } } if arg . index . is_some () { assert ! (arg . is_positional () , "Argument '{}' is a positional argument and can't have short or long name versions" , arg . get_id ()) ; assert ! (arg . is_takes_value_set () , "Argument '{}' is positional and it must take a value but action is {:?}{}" , arg . get_id () , arg . get_action () , if arg . get_id () == Id :: HELP { " (`mut_arg` no longer works with implicit `--help`)" } else if arg . get_id () == Id :: VERSION { " (`mut_arg` no longer works with implicit `--version`)" } else { "" }) ; } let num_vals = arg . get_num_args () . expect (INTERNAL_ERROR_MSG) ; if num_vals != ValueRange :: EMPTY { let num_val_names = arg . get_value_names () . unwrap_or (& []) . len () ; if num_vals . max_values () < num_val_names { panic ! ("Argument {}: Too many value names ({}) compared to `num_args` ({})" , arg . get_id () , num_val_names , num_vals) ; } } assert_eq ! (num_vals . is_multiple () , arg . is_multiple_values_set () , "Argument {}: mismatch between `num_args` ({}) and `multiple_values`" , arg . get_id () , num_vals ,) ; if 1 < num_vals . min_values () { assert ! (! arg . is_require_equals_set () , "Argument {}: cannot accept more than 1 arg (num_args={}) with require_equals" , arg . get_id () , num_vals) ; } if num_vals == ValueRange :: SINGLE { assert ! (! arg . is_multiple_values_set () , "Argument {}: mismatch between `num_args` and `multiple_values`" , arg . get_id ()) ; } assert_arg_flags (arg) ; }
    };
}

assert_arg!()