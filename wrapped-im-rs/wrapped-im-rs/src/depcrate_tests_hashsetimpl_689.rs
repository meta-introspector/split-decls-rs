// Generated macro for impl_689 (impl)
macro_rules! Depcrate_tests_hashsetimpl_689 {
() => {
// Module: crate::tests::hashset
// Provides: {"impl_689"}
// Dependencies: {}
impl < A > Debug for Actions < A > where A : Hash + Eq + Debug + Clone , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { let mut out = String :: new () ; let mut expected = NatSet :: new () ; writeln ! (out , "let mut set = HashSet::new();") ? ; for action in & self . 0 { match action { Action :: Insert (ref value) => { expected . insert (value . clone ()) ; writeln ! (out , "set.insert({:?});" , value) ? ; } Action :: Remove (ref value) => { expected . remove (value) ; writeln ! (out , "set.remove({:?});" , value) ? ; } } } writeln ! (out , "let expected = vec!{:?};" , expected . into_iter () . collect ::< Vec < _ >> ()) ? ; writeln ! (out , "assert_eq!(HashSet::from(expected), set);") ? ; write ! (f , "{}" , super :: code_fmt (& out)) } }
};
}
