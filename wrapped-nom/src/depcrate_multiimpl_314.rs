// Generated macro for impl_314 (impl)
macro_rules! Depcrate_multiimpl_314 {
() => {
// Module: crate::multi
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'a , I , F , O > Parser < I > for Fill < 'a , F , O > where I : Clone , F : Parser < I , Output = O > , { type Output = () ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut input = i . clone () ; for elem in self . buf . iter_mut () { let input_ = input . clone () ; match self . parser . process :: < OM > (input_) { Ok ((i , o)) => { OM :: Output :: map (o , | o | * elem = o) ; input = i ; } Err (Err :: Error (e)) => { return Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: append (i , ErrorKind :: Count , e) }))) ; } Err (e) => { return Err (e) ; } } } Ok ((input , OM :: Output :: bind (| | ()))) } }
};
}
