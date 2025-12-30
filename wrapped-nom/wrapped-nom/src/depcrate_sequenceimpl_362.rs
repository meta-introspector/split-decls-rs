// Generated macro for impl_362 (impl)
macro_rules! Depcrate_sequenceimpl_362 {
() => {
// Module: crate::sequence
// Provides: {"impl_362"}
// Dependencies: {}
impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : Parser < I , Error = E > > Parser < I > for Terminated < F , G > { type Output = < F as Parser < I > > :: Output ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , o1) = self . f . process :: < OM > (i) ? ; let (i , _) = self . g . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i) ? ; Ok ((i , o1)) } }
};
}
