// Generated macro for impl_293 (impl)
macro_rules! Depcrate_multiimpl_293 {
() => {
// Module: crate::multi
// Provides: {"impl_293"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I , F , G , E > Parser < I > for ManyTill < F , G , E > where I : Clone + Input , F : Parser < I , Error = E > , G : Parser < I , Error = E > , E : ParseError < I > , { type Output = (Vec < < F as Parser < I > > :: Output > , < G as Parser < I > > :: Output) ; type Error = E ; fn process < OM : OutputMode > (& mut self , mut i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (crate :: lib :: std :: vec :: Vec :: new) ; loop { let len = i . input_len () ; match self . g . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i . clone ()) { Ok ((i1 , o)) => return Ok ((i1 , OM :: Output :: combine (res , o , | res , o | (res , o)))) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => return Err (Err :: Incomplete (i)) , Err (Err :: Error (_)) => { match self . f . process :: < OM > (i . clone ()) { Err (Err :: Error (err)) => { return Err (Err :: Error (OM :: Error :: map (err , | err | { E :: append (i , ErrorKind :: ManyTill , err) }))) } Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i1 , o)) => { if i1 . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { E :: from_error_kind (i , ErrorKind :: Many0) }))) ; } i = i1 ; res = OM :: Output :: combine (res , o , | mut acc , o | { acc . push (o) ; acc }) } } } } } } }
};
}
