// Generated macro for path_to_str_literal (function)
macro_rules! Depcratepath_to_str_literal {
() => {
// Module: crate
// Provides: {"path_to_str_literal"}
// Dependencies: {}
fn path_to_str_literal < P : AsRef < Path > > (path : P) -> Token { Token :: Literal (Lit :: Str (path . as_ref () . to_str () . unwrap () . to_owned () , StrStyle :: Cooked ,)) }
};
}
