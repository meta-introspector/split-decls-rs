// Generated macro for macro_700 (macro)
macro_rules! Depcrate_tests_ordsetmacro_700 {
() => {
// Module: crate::tests::ordset
// Provides: {"macro_700"}
// Dependencies: {}
proptest ! { # [test] fn comprehensive (actions : Actions < u8 >) { let mut set = OrdSet :: new () ; let mut nat = BTreeSet :: new () ; for action in actions . 0 { match action { Action :: Insert (value) => { let len = nat . len () + if nat . contains (& value) { 0 } else { 1 } ; nat . insert (value) ; set . insert (value) ; assert_eq ! (len , set . len ()) ; } Action :: Remove (value) => { let len = nat . len () - if nat . contains (& value) { 1 } else { 0 } ; nat . remove (& value) ; set . remove (& value) ; assert_eq ! (len , set . len ()) ; } } assert_eq ! (nat . len () , set . len ()) ; assert_eq ! (OrdSet :: from (nat . clone ()) , set) ; assert ! (nat . iter () . eq (set . iter ())) ; } } }
};
}
