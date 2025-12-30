// Generated macro for impl_38 (impl)
macro_rules! Depcrate_precedenceimpl_38 {
() => {
// Module: crate::precedence
// Provides: {"impl_38"}
// Dependencies: {}
impl < I , E , O , OP , G , F , B > Parser < I > for LeftAssoc < F , G , B > where I : Clone + Input , E : ParseError < I > , F : Parser < I , Output = O , Error = E > , G : Parser < I , Output = OP , Error = E > , B : FnMut (O , OP , O) -> O , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , mut i : I ,) -> nom :: PResult < OM , I , Self :: Output , Self :: Error > { let (i1 , mut res) = self . child . process :: < OM > (i) ? ; i = i1 ; loop { let len = i . input_len () ; match self . operator . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i . clone ()) { Err (Err :: Error (_)) => return Ok ((i , res)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i1 , op)) => { match self . child . process :: < OutputM < OM :: Output , Check , OM :: Incomplete > > (i1 . clone ()) { Err (Err :: Error (_)) => return Ok ((i , res)) , Err (Err :: Failure (e)) => return Err (Err :: Failure (e)) , Err (Err :: Incomplete (e)) => return Err (Err :: Incomplete (e)) , Ok ((i2 , rhs)) => { if i2 . input_len () == len { return Err (Err :: Error (OM :: Error :: bind (| | { < F as Parser < I > > :: Error :: from_error_kind (i , ErrorKind :: SeparatedList) }))) ; } let op_rhs = OM :: Output :: combine (op , rhs , | op , rhs | (op , rhs)) ; res = OM :: Output :: combine (res , op_rhs , | lhs , (op , rhs) | (self . builder) (lhs , op , rhs)) ; i = i2 ; } } } } } } }
};
}
