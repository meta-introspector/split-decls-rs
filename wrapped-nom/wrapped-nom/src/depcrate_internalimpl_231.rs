// Generated macro for impl_231 (impl)
macro_rules! Depcrate_internalimpl_231 {
() => {
// Module: crate::internal
// Provides: {"impl_231"}
// Dependencies: {}
impl < I , F : Parser < I > , G : Parser < < F as Parser < I > > :: Output , Error = < F as Parser < I > > :: Error > > Parser < I > for AndThen < F , G > { type Output = < G as Parser < < F as Parser < I > > :: Output > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (input , o1) = self . f . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (i) ? ; let (_ , o2) = self . g . process :: < OM > (o1) ? ; Ok ((input , o2)) } }
};
}
