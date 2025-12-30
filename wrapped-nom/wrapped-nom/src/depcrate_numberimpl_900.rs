// Generated macro for impl_900 (impl)
macro_rules! Depcrate_numberimpl_900 {
() => {
// Module: crate::number
// Provides: {"impl_900"}
// Dependencies: {}
impl < I , Uint , E : ParseError < I > > Parser < I > for LeUint < Uint , E > where I : Input < Item = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , { type Output = Uint ; type Error = E ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { if input . input_len () < self . bound { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (self . bound - input . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | { make_error (input , ErrorKind :: Eof) }))) } } else { let res = OM :: Output :: bind (| | { let mut res = Uint :: default () ; for (index , byte) in input . iter_elements () . take (self . bound) . enumerate () { res = res + (Uint :: from (byte) << (8 * index as u8)) ; } res }) ; Ok ((input . take_from (self . bound) , res)) } } }
};
}
