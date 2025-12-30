// Generated macro for impl_80 (impl)
macro_rules! Depcrate_marshalledimpl_80 {
() => {
// Module: crate::marshalled
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > Iterator for MultiIter < 'a > { type Item = Result < Single < 'a > , DemarshalError > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . sig . single () . map (| (first , rest) | { let mut s = Single { sig : first , data : self . inner . data , start_pos : self . start_pos , is_big_endian : self . inner . is_big_endian , } ; let mut len = s . get_real_length () ? ; if rest . len () > 0 { len = align_up (len + self . start_pos , align_of (rest . as_bytes () [0])) - self . start_pos ; } if len > self . inner . data . len () { Err (DemarshalError :: NotEnoughData) ? } let (fdata , rdata) = self . inner . data . split_at (len) ; s . data = fdata ; self . inner . data = rdata ; self . inner . sig = rest ; self . start_pos += len ; Ok (s) }) } }
};
}
