// Generated macro for impl_317 (impl)
macro_rules! Depcrate_multiimpl_317 {
() => {
// Module: crate::multi
// Provides: {"impl_317"}
// Dependencies: {}
impl < I , F , G , Init , R > Parser < I > for FoldMany0 < F , G , Init , R > where I : Clone + Input , F : Parser < I > , G : FnMut (R , < F as Parser < I > > :: Output) -> R , Init : FnMut () -> R , { type Output = R ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (| | (self . init) ()) ; let mut input = i ; loop { let i_ = input . clone () ; let len = input . input_len () ; match self . parser . process :: < OM > (i_) { Ok ((i , o)) => { if i . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: Many0) }))) ; } res = OM :: Output :: combine (res , o , | res , o | (self . g) (res , o)) ; input = i ; } Err (Err :: Error (_)) => { return Ok ((input , res)) ; } Err (e) => { return Err (e) ; } } } } }
};
}
