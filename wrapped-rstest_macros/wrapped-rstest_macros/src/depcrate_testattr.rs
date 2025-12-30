// Generated macro for attr (function)
macro_rules! Depcrate_testattr {
() => {
// Module: crate::test
// Provides: {"attr"}
// Dependencies: {}
pub (crate) fn attr (s : impl AsRef < str >) -> syn :: Attribute { let a = attrs (s) ; assert_eq ! (1 , a . len ()) ; a . into_iter () . next () . unwrap () }
};
}
