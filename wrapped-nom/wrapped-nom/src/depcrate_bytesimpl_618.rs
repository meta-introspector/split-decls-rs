// Generated macro for impl_618 (impl)
macro_rules! Depcrate_bytesimpl_618 {
() => {
// Module: crate::bytes
// Provides: {"impl_618"}
// Dependencies: {}
impl < I , T , Error : ParseError < I > > Parser < I > for TakeUntil1 < T , Error > where I : Input + FindSubstring < T > , T : Clone , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match i . find_substring (self . tag . clone ()) { None => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: Unknown)) } else { Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: TakeUntil ; Error :: from_error_kind (i , e) }))) } } Some (0) => Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (i , ErrorKind :: TakeUntil) }))) , Some (index) => Ok ((i . take_from (index) , OM :: Output :: bind (| | i . take (index)))) , } } }
};
}
