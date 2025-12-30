// Generated macro for impl_143 (impl)
macro_rules! Depcrate_combinatorimpl_143 {
() => {
// Module: crate::combinator
// Provides: {"impl_143"}
// Dependencies: {}
impl < I , F > Parser < I > for Peek < F > where I : Clone , F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (input) { Ok ((_ , o)) => Ok ((i , o)) , Err (e) => Err (e) , } } }
};
}
