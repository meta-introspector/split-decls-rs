// Generated macro for _try (macro)
macro_rules! Depcrate_to_fmt_try {
() => {
// Module: crate::to_fmt
// Provides: {"_try"}
// Dependencies: {}
macro_rules ! _try { ($ e : expr) => { match ($ e) { Ok (_o) => _o , Err (_) => return Err (sval :: Error :: new ()) , } } ; }
};
}
