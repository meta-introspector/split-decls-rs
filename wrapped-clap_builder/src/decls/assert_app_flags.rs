macro_rules! deps {
    () => {
        Command!();
        AppSettings!();
    };
}

macro_rules! assert_app_flags {
    () => {
        deps!();
        fn assert_app_flags (cmd : & Command) { macro_rules ! checker { ($ a : ident conflicts $ ($ b : ident) |+) => { if cmd .$ a () { let mut s = String :: new () ; $ (if cmd .$ b () { use std :: fmt :: Write ; write ! (& mut s , "  AppSettings::{} conflicts with AppSettings::{}.\n" , std :: stringify ! ($ b) , std :: stringify ! ($ a)) . unwrap () ; }) + if ! s . is_empty () { panic ! ("{}\n{}" , cmd . get_name () , s) } } } ; } checker ! (is_multicall_set conflicts is_no_binary_name_set) ; }
    };
}

assert_app_flags!();