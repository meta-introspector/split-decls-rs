// Generated macro for find_cap_ref_braced (function)
macro_rules! Depcrate_util_interpolatefind_cap_ref_braced {
() => {
// Module: crate::util::interpolate
// Provides: {"find_cap_ref_braced"}
// Dependencies: {}
# [doc = " Looks for a braced reference, e.g., `${foo1}`. This assumes that an opening"] # [doc = " brace has been found at `i-1` in `rep`. This then looks for a closing"] # [doc = " brace and returns the capture reference within the brace."] fn find_cap_ref_braced (rep : & [u8] , mut i : usize) -> Option < CaptureRef < '_ > > { assert_eq ! (b'{' , rep [i . checked_sub (1) . unwrap ()]) ; let start = i ; while rep . get (i) . map_or (false , | & b | b != b'}') { i += 1 ; } if ! rep . get (i) . map_or (false , | & b | b == b'}') { return None ; } let cap = match core :: str :: from_utf8 (& rep [start .. i]) { Err (_) => return None , Ok (cap) => cap , } ; Some (CaptureRef { cap : match cap . parse :: < usize > () { Ok (i) => Ref :: Number (i) , Err (_) => Ref :: Named (cap) , } , end : i + 1 , }) }
};
}
