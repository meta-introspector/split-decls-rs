// Generated macro for find_fwd (function)
macro_rules! Depcrate_dfa_accelfind_fwd {
() => {
// Module: crate::dfa::accel
// Provides: {"find_fwd"}
// Dependencies: {}
# [doc = " Search for between 1 and 3 needle bytes in the given haystack, starting the"] # [doc = " search at the given position. If `needles` has a length other than 1-3,"] # [doc = " then this panics."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn find_fwd (needles : & [u8] , haystack : & [u8] , at : usize ,) -> Option < usize > { let bs = needles ; let i = match needles . len () { 1 => memchr :: memchr (bs [0] , & haystack [at ..]) ? , 2 => memchr :: memchr2 (bs [0] , bs [1] , & haystack [at ..]) ? , 3 => memchr :: memchr3 (bs [0] , bs [1] , bs [2] , & haystack [at ..]) ? , 0 => panic ! ("cannot find with empty needles") , n => panic ! ("invalid needles length: {}" , n) , } ; Some (at + i) }
};
}
