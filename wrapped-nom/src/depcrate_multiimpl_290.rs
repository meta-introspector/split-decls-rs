// Generated macro for impl_290 (impl)
macro_rules! Depcrate_multiimpl_290 {
() => {
// Module: crate::multi
// Provides: {"impl_290"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I , F > Parser < I > for Many1 < F > where I : Clone + Input , F : Parser < I > , { type Output = Vec < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , mut i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match self . parser . process :: < OutputM < OM :: Output , Emit , OM :: Incomplete > > (i . clone ()) { Err (Err :: Error (err)) => Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: append (i , ErrorKind :: Many1 , err) }))) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Ok ((i1 , o)) => { let mut acc = OM :: Output :: map (o , | o | { let mut acc = crate :: lib :: std :: vec :: Vec :: with_capacity (4) ; acc . push (o) ; acc }) ; i = i1 ; loop { let len = i . input_len () ; match self . parser . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i . clone ()) { Err (Err :: Error (_)) => return Ok ((i , acc)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i1 , o)) => { if i1 . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: Many0) }))) ; } i = i1 ; acc = OM :: Output :: combine (acc , o , | mut acc , o | { acc . push (o) ; acc }) } } } } } } }
};
}
