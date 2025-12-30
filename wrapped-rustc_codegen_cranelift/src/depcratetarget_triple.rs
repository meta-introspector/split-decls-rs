// Generated macro for target_triple (function)
macro_rules! Depcratetarget_triple {
() => {
// Module: crate
// Provides: {"target_triple"}
// Dependencies: {}
fn target_triple (sess : & Session) -> target_lexicon :: Triple { match sess . target . llvm_target . parse () { Ok (triple) => triple , Err (err) => sess . dcx () . fatal (format ! ("target not recognized: {}" , err)) , } }
};
}
