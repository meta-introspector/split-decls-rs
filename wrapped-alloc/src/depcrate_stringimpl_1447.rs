// Generated macro for impl_1447 (impl)
macro_rules! Depcrate_stringimpl_1447 {
() => {
// Module: crate::string
// Provides: {"impl_1447"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (feature = "ascii_char" , issue = "110998")] impl Extend < core :: ascii :: Char > for String { # [inline] # [track_caller] fn extend < I : IntoIterator < Item = core :: ascii :: Char > > (& mut self , iter : I) { self . vec . extend (iter . into_iter () . map (| c | c . to_u8 ())) ; } # [inline] # [track_caller] fn extend_one (& mut self , c : core :: ascii :: Char) { self . vec . push (c . to_u8 ()) ; } }
};
}
