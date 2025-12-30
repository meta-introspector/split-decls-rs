// Generated macro for p (macro)
macro_rules! Depcrate_testp {
() => {
// Module: crate::test
// Provides: {"p"}
// Dependencies: {}
macro_rules ! p { ($ e : expr) => { match $ e { Ok (r) => r , Err (e) => panic ! ("{:?}" , e) , } } ; }
};
}
