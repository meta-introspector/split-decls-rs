// Generated macro for t (macro)
macro_rules! Depcrate_testt {
() => {
// Module: crate::test
// Provides: {"t"}
// Dependencies: {}
macro_rules ! t { ($ e : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {}" , stringify ! ($ e) , e) , } } ; }
};
}
