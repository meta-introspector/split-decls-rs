// Generated macro for panictry (macro)
macro_rules! Depcratepanictry {
() => {
// Module: crate
// Provides: {"panictry"}
// Dependencies: {}
macro_rules ! panictry { ($ e : expr) => ({ use syntax :: diagnostic :: FatalError ; match $ e { Ok (e) => e , Err (FatalError) => panic ! (FatalError) } }) }
};
}
