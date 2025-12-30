// Generated macro for impl_619 (impl)
macro_rules! Depcrate_parser_errorimpl_619 {
() => {
// Module: crate::parser::error
// Provides: {"impl_619"}
// Dependencies: {}
impl MatchesError { # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn unwrap < T > (id : & str , r : Result < T , MatchesError >) -> T { let err = match r { Ok (t) => { return t ; } Err (err) => err , } ; panic ! ("Mismatch between definition and access of `{id}`. {err}" ,) } }
};
}
