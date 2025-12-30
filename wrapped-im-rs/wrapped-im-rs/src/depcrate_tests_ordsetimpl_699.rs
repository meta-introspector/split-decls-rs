// Generated macro for impl_699 (impl)
macro_rules! Depcrate_tests_ordsetimpl_699 {
() => {
// Module: crate::tests::ordset
// Provides: {"impl_699"}
// Dependencies: {}
impl < A > Debug for Actions < A > where A : Ord + Debug + Clone , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { let mut out = String :: new () ; let mut expected = BTreeSet :: new () ; writeln ! (out , "let mut set = OrdSet::new();") ? ; for action in & self . 0 { match action { Action :: Insert (ref value) => { expected . insert (value . clone ()) ; writeln ! (out , "set.insert({:?});" , value) ? ; } Action :: Remove (ref value) => { expected . remove (value) ; writeln ! (out , "set.remove({:?});" , value) ? ; } } } writeln ! (out , "let expected = vec!{:?};" , expected . into_iter () . collect ::< Vec < _ >> ()) ? ; writeln ! (out , "assert_eq!(OrdSet::from(expected), set);") ? ; write ! (f , "{}" , super :: code_fmt (& out)) } }
};
}
