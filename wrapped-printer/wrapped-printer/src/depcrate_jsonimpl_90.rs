// Generated macro for impl_90 (impl)
macro_rules! Depcrate_jsonimpl_90 {
() => {
// Module: crate::json
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > SubMatches < 'a > { # [doc = " Create a new set of match ranges from a set of matches and the"] # [doc = " corresponding bytes that those matches apply to."] fn new (bytes : & 'a [u8] , matches : & [Match] , replacement : Option < (& 'a [u8] , & 'a [Match]) > ,) -> SubMatches < 'a > { if matches . len () == 1 { let mat = matches [0] ; SubMatches :: Small ([jsont :: SubMatch { m : & bytes [mat] , replacement : replacement . map (| (rbuf , rmatches) | & rbuf [rmatches [0]]) , start : mat . start () , end : mat . end () , }]) } else { let mut match_ranges = vec ! [] ; for (i , & mat) in matches . iter () . enumerate () { match_ranges . push (jsont :: SubMatch { m : & bytes [mat] , replacement : replacement . map (| (rbuf , rmatches) | & rbuf [rmatches [i]]) , start : mat . start () , end : mat . end () , }) ; } SubMatches :: Big (match_ranges) } } # [doc = " Create an empty set of match ranges."] fn empty () -> SubMatches < 'static > { SubMatches :: Empty } # [doc = " Return this set of match ranges as a slice."] fn as_slice (& self) -> & [jsont :: SubMatch < '_ >] { match * self { SubMatches :: Empty => & [] , SubMatches :: Small (ref x) => x , SubMatches :: Big (ref x) => x , } } }
};
}
