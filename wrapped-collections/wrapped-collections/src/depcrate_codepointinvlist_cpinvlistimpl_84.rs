// Generated macro for impl_84 (impl)
macro_rules! Depcrate_codepointinvlist_cpinvlistimpl_84 {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for CodePointInversionList < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { use serde :: ser :: Error ; use serde :: ser :: SerializeSeq ; let mut seq = serializer . serialize_seq (Some (self . inv_list . len () / 2)) ? ; for range in self . iter_ranges () { let start = UnicodeCodePoint :: from_u32 (* range . start ()) . map_err (S :: Error :: custom) ? ; if range . start () == range . end () { seq . serialize_element (& format ! ("{start}")) ? ; } else { let end = UnicodeCodePoint :: from_u32 (* range . end ()) . map_err (S :: Error :: custom) ? ; seq . serialize_element (& format ! ("{start}-{end}" ,)) ? ; } } seq . end () } else { self . inv_list . serialize (serializer) } } }
};
}
