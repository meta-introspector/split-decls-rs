// Generated macro for impl_157 (impl)
macro_rules! Depcrate_combinatorimpl_157 {
() => {
// Module: crate::combinator
// Provides: {"impl_157"}
// Dependencies: {}
impl < I , F > Parser < I > for Not < F > where I : Clone , F : Parser < I > , { type Output = () ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (input) { Ok (_) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Not) }))) , Err (Err :: Error (_)) => Ok ((i , OM :: Output :: bind (| | ()))) , Err (e) => Err (e) , } } }
};
}
