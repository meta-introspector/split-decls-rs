macro_rules! testable_unit {
    () => {
        # [test] fn testable_unit () { fn do_nothing () { } quickcheck (do_nothing as fn ()) ; }
    };
}

testable_unit!();