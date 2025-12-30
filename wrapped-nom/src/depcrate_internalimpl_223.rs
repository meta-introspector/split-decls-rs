// Generated macro for impl_223 (impl)
macro_rules! Depcrate_internalimpl_223 {
() => {
// Module: crate::internal
// Provides: {"impl_223"}
// Dependencies: {}
impl < I , O2 , E : ParseError < I > , F : Parser < I , Error = E > , G : FnMut (< F as Parser < I > > :: Output) -> O2 > Parser < I > for Map < F , G > { type Output = O2 ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match self . f . process :: < OM > (i) { Err (e) => Err (e) , Ok ((i , o)) => Ok ((i , OM :: Output :: map (o , | o | (self . g) (o)))) , } } }
};
}
