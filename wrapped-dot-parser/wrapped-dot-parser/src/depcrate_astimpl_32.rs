// Generated macro for impl_32 (impl)
macro_rules! Depcrate_astimpl_32 {
() => {
// Module: crate::ast
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Graph < (ID < 'a > , ID < 'a >) > { type Error = PestError ; fn try_from (s : & 'a str) -> Result < Self , PestError > { let mut pairs = DotParser :: parse (Rule :: dotgraph , s) ? ; match pairs . next () { None => { panic ! ("The toplevel `Pairs` is empty.") } Some (pair) => match Graph :: try_from (pair) { Ok (g) => Ok (g) , Err (e) => { panic ! ("{}" , e) ; } } , } } }
};
}
