macro_rules! deps {
    () => {
        QuickCheck!();
    };
}

macro_rules! prop_oob {
    () => {
        deps!();
        # [test] fn prop_oob () { fn prop () -> bool { let zero : Vec < bool > = vec ! [] ; zero [0] } if let Ok (n) = QuickCheck :: new () . quicktest (prop as fn () -> bool) { panic ! ("prop_oob should fail with a runtime error \
            but instead it passed {} tests." , n) ; } }
    };
}

prop_oob!();