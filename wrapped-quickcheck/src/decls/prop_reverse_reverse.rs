macro_rules! prop_reverse_reverse {
    () => {
        # [test] fn prop_reverse_reverse () { fn prop (xs : Vec < usize >) -> bool { let rev : Vec < _ > = xs . clone () . into_iter () . rev () . collect () ; let revrev : Vec < _ > = rev . into_iter () . rev () . collect () ; xs == revrev } quickcheck (prop as fn (Vec < usize >) -> bool) ; }
    };
}

prop_reverse_reverse!();