// Generated macro for impl_78 (impl)
macro_rules! Depcrate_codepointinvlist_cpinvlistimpl_78 {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de : 'a , 'a > serde :: Deserialize < 'de > for CodePointInversionList < 'a > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; let parsed_inv_list = if deserializer . is_human_readable () { let parsed_strings = Vec :: < alloc :: borrow :: Cow < 'de , str > > :: deserialize (deserializer) ? ; let mut inv_list = ZeroVec :: new_owned (Vec :: with_capacity (parsed_strings . len () * 2)) ; for range in parsed_strings { fn internal (range : & str) -> Option < (u32 , u32) > { let (start , range) = UnicodeCodePoint :: parse (range) ? ; if range . is_empty () { return Some ((start . 0 , start . 0)) ; } let (hyphen , range) = UnicodeCodePoint :: parse (range) ? ; if hyphen . 0 != '-' as u32 { return None ; } let (end , range) = UnicodeCodePoint :: parse (range) ? ; range . is_empty () . then_some ((start . 0 , end . 0)) } let (start , end) = internal (& range) . ok_or_else (| | Error :: custom (format ! ("Cannot deserialize invalid inversion list for CodePointInversionList: {range:?}"))) ? ; inv_list . with_mut (| v | { v . push (PotentialCodePoint :: from_u24 (start) . to_unaligned ()) ; v . push (PotentialCodePoint :: from_u24 (end + 1) . to_unaligned ()) ; }) ; } inv_list } else { ZeroVec :: < PotentialCodePoint > :: deserialize (deserializer) ? } ; CodePointInversionList :: try_from_inversion_list (parsed_inv_list) . map_err (| e | { Error :: custom (format ! ("Cannot deserialize invalid inversion list for CodePointInversionList: {e:?}")) }) } }
};
}
