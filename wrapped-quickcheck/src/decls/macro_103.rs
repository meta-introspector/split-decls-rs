macro_rules! macro_103 {
    () => {
        quickcheck ! { fn prop_reverse_reverse_macro (xs : Vec < usize >) -> bool { let rev : Vec < _ > = xs . clone () . into_iter () . rev () . collect () ; let revrev : Vec < _ > = rev . into_iter () . rev () . collect () ; xs == revrev } # [should_panic] fn prop_macro_panic (_x : u32) -> bool { assert ! (false) ; false } }
    };
}

macro_103!()