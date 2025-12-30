// Generated macro for impl_163 (impl)
macro_rules! Depcrate_combinatorimpl_163 {
() => {
// Module: crate::combinator
// Provides: {"impl_163"}
// Dependencies: {}
impl < I , F > Parser < I > for Consumed < F > where I : Clone + Offset + Input , F : Parser < I > , { type Output = (I , < F as Parser < I > > :: Output) ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (i) { Ok ((remaining , result)) => { let index = input . offset (& remaining) ; Ok ((remaining , OM :: Output :: map (result , | res | { let consumed = input . take (index) ; (consumed , res) }) ,)) } Err (e) => Err (e) , } } }
};
}
