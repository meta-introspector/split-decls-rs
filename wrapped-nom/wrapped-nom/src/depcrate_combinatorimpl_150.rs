// Generated macro for impl_150 (impl)
macro_rules! Depcrate_combinatorimpl_150 {
() => {
// Module: crate::combinator
// Provides: {"impl_150"}
// Dependencies: {}
impl < I , F > Parser < I > for AllConsuming < F > where I : Input , F : Parser < I > , { type Output = < F as Parser < I > > :: Output ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let (input , res) = self . parser . process :: < OM > (input) ? ; if input . input_len () == 0 { Ok ((input , res)) } else { Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Eof) }))) } } }
};
}
