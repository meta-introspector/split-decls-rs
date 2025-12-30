// Generated macro for impl_1448 (impl)
macro_rules! Depcrate_stringimpl_1448 {
() => {
// Module: crate::string
// Provides: {"impl_1448"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (feature = "ascii_char" , issue = "110998")] impl < 'a > Extend < & 'a core :: ascii :: Char > for String { # [inline] # [track_caller] fn extend < I : IntoIterator < Item = & 'a core :: ascii :: Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] # [track_caller] fn extend_one (& mut self , c : & 'a core :: ascii :: Char) { self . vec . push (c . to_u8 ()) ; } }
};
}
