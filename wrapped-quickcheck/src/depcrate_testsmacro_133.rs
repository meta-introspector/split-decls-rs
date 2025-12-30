// Generated macro for macro_133 (macro)
macro_rules! Depcrate_testsmacro_133 {
() => {
// Module: crate::tests
// Provides: {"macro_133"}
// Dependencies: {}
quickcheck ! { fn prop_reverse_reverse_macro (xs : Vec < usize >) -> bool { let rev : Vec < _ > = xs . clone () . into_iter () . rev () . collect () ; let revrev : Vec < _ > = rev . into_iter () . rev () . collect () ; xs == revrev } # [should_panic] fn prop_macro_panic (_x : u32) -> bool { assert ! (false) ; false } }
};
}
