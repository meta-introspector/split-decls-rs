// Generated macro for impl_22 (impl)
macro_rules! Depcrate_cfgimpl_22 {
() => {
// Module: crate::cfg
// Provides: {"impl_22"}
// Dependencies: {}
impl FromStr for Cfg { type Err = ParseError ; fn from_str (s : & str) -> Result < Cfg , Self :: Err > { let mut p = Parser :: new (s) ; let e = p . cfg () ? ; if let Some (rest) = p . rest () { return Err (ParseError :: new (p . t . orig , UnterminatedExpression (rest . to_string ()) ,)) ; } Ok (e) } }
};
}
