// Generated macro for impl_147 (impl)
macro_rules! Depcrate_combinatorimpl_147 {
() => {
// Module: crate::combinator
// Provides: {"impl_147"}
// Dependencies: {}
impl < I , F > Parser < I > for MakeComplete < F > where I : Clone , F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OutputM < OM :: Output , OM :: Error , Complete > > (input) { Err (Err :: Incomplete (_)) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Complete) }))) , Err (e) => Err (e) , Ok (o) => Ok (o) , } } }
};
}
