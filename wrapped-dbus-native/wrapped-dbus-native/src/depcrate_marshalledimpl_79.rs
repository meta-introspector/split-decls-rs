// Generated macro for impl_79 (impl)
macro_rules! Depcrate_marshalledimpl_79 {
() => {
// Module: crate::marshalled
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > Multi < 'a > { pub fn new (sig : & 'a SignatureMulti , data : & 'a [u8] , is_big_endian : bool) -> Self { Multi { sig , data , is_big_endian } } fn get_real_length (& self) -> Result < usize , DemarshalError > { let x = self . data . len () ; let mut iter = self . iter () ; while let Some (r) = iter . next () { r ? ; } Ok (x - iter . inner . data . len ()) } pub fn iter (& self) -> MultiIter < 'a > { MultiIter { inner : * self , start_pos : 0 } } }
};
}
