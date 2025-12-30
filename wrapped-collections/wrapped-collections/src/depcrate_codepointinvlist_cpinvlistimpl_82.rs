// Generated macro for impl_82 (impl)
macro_rules! Depcrate_codepointinvlist_cpinvlistimpl_82 {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "serde")] impl UnicodeCodePoint { fn from_u32 (cp : u32) -> Result < Self , String > { if cp <= char :: MAX as u32 { Ok (Self (cp)) } else { Err (format ! ("Not a Unicode code point {cp}")) } } fn parse (value : & str) -> Option < (Self , & str) > { Some (if let Some (hex) = value . strip_prefix ("U+") { let (escape , remainder) = (hex . get (.. 4) ? , hex . get (4 ..) ?) ; (Self (u32 :: from_str_radix (escape , 16) . ok () ?) , remainder) } else { let c = value . chars () . next () ? ; (Self (c as u32) , value . get (c . len_utf8 () ..) ?) }) } }
};
}
