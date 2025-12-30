// Generated macro for find_rev (function)
macro_rules! Depcrate_dfa_accelfind_rev {
() => {
// Module: crate::dfa::accel
// Provides: {"find_rev"}
// Dependencies: {}
# [doc = " Search for between 1 and 3 needle bytes in the given haystack in reverse,"] # [doc = " starting the search at the given position. If `needles` has a length other"] # [doc = " than 1-3, then this panics."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn find_rev (needles : & [u8] , haystack : & [u8] , at : usize ,) -> Option < usize > { let bs = needles ; match needles . len () { 1 => memchr :: memrchr (bs [0] , & haystack [.. at]) , 2 => memchr :: memrchr2 (bs [0] , bs [1] , & haystack [.. at]) , 3 => memchr :: memrchr3 (bs [0] , bs [1] , bs [2] , & haystack [.. at]) , 0 => panic ! ("cannot find with empty needles") , n => panic ! ("invalid needles length: {}" , n) , } }
};
}
