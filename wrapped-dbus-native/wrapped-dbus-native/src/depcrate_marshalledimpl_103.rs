// Generated macro for impl_103 (impl)
macro_rules! Depcrate_marshalledimpl_103 {
() => {
// Module: crate::marshalled
// Provides: {"impl_103"}
// Dependencies: {}
impl MultiBuf { pub fn new () -> Self { Default :: default () } pub fn multi (& self) -> Multi { Multi { sig : & self . sig , data : & self . data , is_big_endian : IS_BIG_ENDIAN } } pub fn append < T : Marshal + ? Sized > (& mut self , value : & T) -> Result < () , DemarshalError > { let new_sig = value . signature () ; if self . sig . len () + new_sig . len () > 255 { return Err (DemarshalError :: NumberTooBig) } let temp = mem :: replace (& mut self . sig , Default :: default ()) ; let mut temp = temp . into_inner () ; temp . push_str (new_sig) ; debug_assert ! (SignatureMulti :: is_valid (& temp) . is_ok ()) ; self . sig = SignatureMulti :: new_unchecked_owned (temp) ; value . append_data_to (& mut self . data) ; Ok (()) } pub fn into_inner (self) -> (SignatureMultiBuf , Vec < u8 >) { (self . sig , self . data) } }
};
}
