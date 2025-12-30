// Generated macro for impl_233 (impl)
macro_rules! Depcrate_internalimpl_233 {
() => {
// Module: crate::internal
// Provides: {"impl_233"}
// Dependencies: {}
impl < I , E : ParseError < I > , F : Parser < I , Error = E > , G : Parser < I , Error = E > > Parser < I > for And < F , G > { type Output = (< F as Parser < I > > :: Output , < G as Parser < I > > :: Output) ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (i , o1) = self . f . process :: < OM > (i) ? ; let (i , o2) = self . g . process :: < OM > (i) ? ; Ok ((i , OM :: Output :: combine (o1 , o2 , | o1 , o2 | (o1 , o2)))) } }
};
}
