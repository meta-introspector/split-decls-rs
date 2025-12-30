// Generated macro for impl_37 (impl)
macro_rules! Depcrate_matchingimpl_37 {
() => {
// Module: crate::matching
// Provides: {"impl_37"}
// Dependencies: {}
impl NodeKind { fn matches (& self , node : & SyntaxNode) -> Result < () , MatchFailed > { let ok = match self { Self :: Literal => { cov_mark :: hit ! (literal_constraint) ; ast :: Literal :: can_cast (node . kind ()) } } ; if ! ok { fail_match ! ("Code '{}' isn't of kind {:?}" , node . text () , self) ; } Ok (()) } }
};
}
