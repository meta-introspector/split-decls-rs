macro_rules! remove_period {
    () => {
        fn remove_period (mut s : String) -> String { if s . ends_with ('.') && ! s . ends_with ("..") { s . pop () ; } s }
    };
}

remove_period!();