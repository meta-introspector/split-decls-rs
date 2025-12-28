macro_rules! deps {
    () => {
        QuickCheck!();
    };
}

macro_rules! testable_unit_panic {
    () => {
        deps!();
        # [test] fn testable_unit_panic () { fn panic () { panic ! () ; } assert ! (QuickCheck :: new () . quicktest (panic as fn ()) . is_err ()) ; }
    };
}

testable_unit_panic!()