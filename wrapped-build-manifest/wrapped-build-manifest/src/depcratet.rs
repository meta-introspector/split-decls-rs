// Generated macro for t (macro)
macro_rules! Depcratet {
() => {
// Module: crate
// Provides: {"t"}
// Dependencies: {}
macro_rules ! t { ($ e : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {}" , stringify ! ($ e) , e) , } } ; ($ e : expr , $ extra : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {}: {}" , stringify ! ($ e) , e , $ extra) , } } ; }
};
}
