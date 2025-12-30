// Generated macro for uppercase (function)
macro_rules! Depcrateuppercase {
() => {
// Module: crate
// Provides: {"uppercase"}
// Dependencies: {}
fn uppercase (s : & str , f : & mut fmt :: Formatter) -> fmt :: Result { for c in s . chars () { write ! (f , "{}" , c . to_uppercase ()) ? ; } Ok (()) }
};
}
