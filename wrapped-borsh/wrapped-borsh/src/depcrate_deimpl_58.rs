// Generated macro for impl_58 (impl)
macro_rules! Depcrate_deimpl_58 {
() => {
// Module: crate::de
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > BorshDeserialize for Vec < T > where T : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < T > () ? ; let len = u32 :: deserialize_reader (reader) ? ; if len == 0 { Ok (Vec :: new ()) } else if let Some (vec_bytes) = T :: vec_from_reader (len , reader) ? { Ok (vec_bytes) } else { let mut result = Vec :: with_capacity (hint :: cautious :: < T > (len)) ; for _ in 0 .. len { result . push (T :: deserialize_reader (reader) ?) ; } Ok (result) } } }
};
}
