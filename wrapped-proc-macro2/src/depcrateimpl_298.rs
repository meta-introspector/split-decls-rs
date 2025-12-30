// Generated macro for impl_298 (impl)
macro_rules! Depcrateimpl_298 {
() => {
// Module: crate
// Provides: {"impl_298"}
// Dependencies: {}
impl FromStr for Literal { type Err = LexError ; fn from_str (repr : & str) -> Result < Self , LexError > { match imp :: Literal :: from_str_checked (repr) { Ok (lit) => Ok (Literal :: _new (lit)) , Err (lex) => Err (LexError { inner : lex , _marker : MARKER , }) , } } }
};
}
