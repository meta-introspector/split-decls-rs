// Generated macro for impl_239 (impl)
macro_rules! Depcrate_internalimpl_239 {
() => {
// Module: crate::internal
// Provides: {"impl_239"}
// Dependencies: {}
impl < I , F : Parser < I > , G : Parser < I , Output = < F as Parser < I > > :: Output , Error = < F as Parser < I > > :: Error > , > Parser < I > for Either < F , G > { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; # [inline] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self { Either :: Left (f) => f . process :: < OM > (i) , Either :: Right (g) => g . process :: < OM > (i) , } } }
};
}
