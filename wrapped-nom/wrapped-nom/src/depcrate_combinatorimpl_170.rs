// Generated macro for impl_170 (impl)
macro_rules! Depcrate_combinatorimpl_170 {
() => {
// Module: crate::combinator
// Provides: {"impl_170"}
// Dependencies: {}
impl < I : Clone , E , F > ParserIterator < I , E , F > { # [doc = " Returns the remaining input if parsing was successful, or the error if we encountered an error."] pub fn finish (mut self) -> IResult < I , () , E > { match self . state . take () . unwrap () { State :: Running | State :: Done => Ok ((self . input , ())) , State :: Failure (e) => Err (Err :: Failure (e)) , State :: Incomplete (i) => Err (Err :: Incomplete (i)) , } } }
};
}
