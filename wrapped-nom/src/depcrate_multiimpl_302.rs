// Generated macro for impl_302 (impl)
macro_rules! Depcrate_multiimpl_302 {
() => {
// Module: crate::multi
// Provides: {"impl_302"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I , F > Parser < I > for ManyMN < F > where I : Clone + Input , F : Parser < I > , { type Output = Vec < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if self . min > self . max { return Err (Err :: Failure (< F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: ManyMN ,))) ; } let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES / crate :: lib :: std :: mem :: size_of :: < < F as Parser < I > > :: Output > () . max (1) ; let mut res = OM :: Output :: bind (| | { crate :: lib :: std :: vec :: Vec :: with_capacity (self . min . min (max_initial_capacity)) }) ; for count in 0 .. self . max { let len = input . input_len () ; match self . parser . process :: < OM > (input . clone ()) { Ok ((tail , value)) => { if tail . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (input , ErrorKind :: ManyMN) }))) ; } res = OM :: Output :: combine (res , value , | mut res , value | { res . push (value) ; res }) ; input = tail ; } Err (Err :: Error (e)) => { if count < self . min { return Err (Err :: Error (OM :: Error :: map (e , | e | { < F as Parser < I > > :: Error :: append (input , ErrorKind :: ManyMN , e) }))) ; } else { return Ok ((input , res)) ; } } Err (e) => { return Err (e) ; } } } Ok ((input , res)) } }
};
}
