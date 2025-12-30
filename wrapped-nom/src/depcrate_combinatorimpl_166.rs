// Generated macro for impl_166 (impl)
macro_rules! Depcrate_combinatorimpl_166 {
() => {
// Module: crate::combinator
// Provides: {"impl_166"}
// Dependencies: {}
impl < I , F > Parser < I > for Cut < F > where F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . parser . process :: < OutputM < OM :: Output , Emit , OM :: Incomplete > > (input) { Err (Err :: Error (e)) => Err (Err :: Failure (e)) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Ok ((i , o)) => Ok ((i , o)) , } } }
};
}
