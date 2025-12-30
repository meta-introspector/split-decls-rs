// Generated macro for impl_140 (impl)
macro_rules! Depcrate_combinatorimpl_140 {
() => {
// Module: crate::combinator
// Provides: {"impl_140"}
// Dependencies: {}
impl < I , F > Parser < I > for Cond < F > where F : Parser < I > , { type Output = Option < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match & mut self . parser { None => Ok ((input , OM :: Output :: bind (| | None))) , Some (f) => f . process :: < OM > (input) . map (| (i , o) | (i , OM :: Output :: map (o , Some))) , } } }
};
}
