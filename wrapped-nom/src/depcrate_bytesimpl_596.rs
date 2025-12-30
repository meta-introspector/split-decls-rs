// Generated macro for impl_596 (impl)
macro_rules! Depcrate_bytesimpl_596 {
() => {
// Module: crate::bytes
// Provides: {"impl_596"}
// Dependencies: {}
impl < I , Error : ParseError < I > , T > Parser < I > for TagNoCase < T , Error > where I : Input + Compare < T > , T : Input + Clone , { type Output = I ; type Error = Error ; fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let tag_len = self . tag . input_len () ; let t = self . tag . clone () ; match i . compare_no_case (t) { CompareResult :: Ok => Ok ((i . take_from (tag_len) , OM :: Output :: bind (| | i . take (tag_len)))) , CompareResult :: Incomplete => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (tag_len - i . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: Tag ; Error :: from_error_kind (i , e) }))) } } CompareResult :: Error => Err (Err :: Error (OM :: Error :: bind (| | { let e : ErrorKind = ErrorKind :: Tag ; Error :: from_error_kind (i , e) }))) , } } }
};
}
