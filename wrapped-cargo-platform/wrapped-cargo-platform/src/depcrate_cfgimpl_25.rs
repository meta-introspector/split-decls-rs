// Generated macro for impl_25 (impl)
macro_rules! Depcrate_cfgimpl_25 {
() => {
// Module: crate::cfg
// Provides: {"impl_25"}
// Dependencies: {}
impl FromStr for CfgExpr { type Err = ParseError ; fn from_str (s : & str) -> Result < CfgExpr , Self :: Err > { let mut p = Parser :: new (s) ; let e = p . expr () ? ; if let Some (rest) = p . rest () { return Err (ParseError :: new (p . t . orig , UnterminatedExpression (rest . to_string ()) ,)) ; } Ok (e) } }
};
}
