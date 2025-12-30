// Generated macro for macro_690 (macro)
macro_rules! Depcrate_tests_hashsetmacro_690 {
() => {
// Module: crate::tests::hashset
// Provides: {"macro_690"}
// Dependencies: {}
proptest ! { # [test] fn comprehensive (actions : Actions < u8 >) { let mut set = HashSet :: new () ; let mut nat = NatSet :: new () ; for action in actions . 0 { match action { Action :: Insert (value) => { let len = nat . len () + if nat . contains (& value) { 0 } else { 1 } ; nat . insert (value) ; set . insert (value) ; assert_eq ! (len , set . len ()) ; } Action :: Remove (value) => { let len = nat . len () - if nat . contains (& value) { 1 } else { 0 } ; nat . remove (& value) ; set . remove (& value) ; assert_eq ! (len , set . len ()) ; } } assert_eq ! (nat . len () , set . len ()) ; assert_eq ! (HashSet :: from (nat . clone ()) , set) ; } } }
};
}
