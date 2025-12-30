// Generated macro for impl_178 (impl)
macro_rules! Depcrate_combinatorimpl_178 {
() => {
// Module: crate::combinator
// Provides: {"impl_178"}
// Dependencies: {}
impl < I , O , E > Parser < I > for Fail < O , E > where E : ParseError < I > , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { Err (Err :: Error (OM :: Error :: bind (| | { E :: from_error_kind (input , ErrorKind :: Fail) }))) } }
};
}
