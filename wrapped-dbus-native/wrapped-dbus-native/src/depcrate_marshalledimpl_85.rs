// Generated macro for impl_85 (impl)
macro_rules! Depcrate_marshalledimpl_85 {
() => {
// Module: crate::marshalled
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'a > Iterator for Array < 'a > { type Item = Result < Single < 'a > , DemarshalError > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . len () == 0 { return None ; } let mut s = Single { is_big_endian : self . is_big_endian , data : self . data , start_pos : self . start_pos , sig : self . inner_sig , } ; let mut len = match s . get_real_length () { Ok (len) if len <= self . data . len () => len , _ => return Some (Err (DemarshalError :: NotEnoughData)) , } ; s . data = & s . data [0 .. len] ; if len < s . data . len () { len = align_up (len + self . start_pos , align_of (self . inner_sig . as_bytes () [0])) - self . start_pos ; self . start_pos += len ; self . data = & self . data [len ..] ; } else { self . data = & [] ; } Some (Ok (s)) } }
};
}
