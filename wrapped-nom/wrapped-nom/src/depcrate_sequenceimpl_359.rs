// Generated macro for impl_359 (impl)
macro_rules! Depcrate_sequenceimpl_359 {
() => {
// Module: crate::sequence
// Provides: {"impl_359"}
// Dependencies: {}
impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : Parser < I , Error = E > > Parser < I > for Preceded < F , G > { type Output = < G as Parser < I > > :: Output ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , _) = self . f . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i) ? ; let (i , o2) = self . g . process :: < OM > (i) ? ; Ok ((i , o2)) } }
};
}
