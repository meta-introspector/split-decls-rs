// Generated macro for impl_404 (impl)
macro_rules! Depcrate_meta_regeximpl_404 {
() => {
// Module: crate::meta::regex
// Provides: {"impl_404"}
// Dependencies: {}
impl < 'r , 'h > Iterator for CapturesMatches < 'r , 'h > { type Item = Captures ; # [inline] fn next (& mut self) -> Option < Captures > { let CapturesMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; let _ = it . advance (| input | { re . search_captures_with (cache , input , caps) ; Ok (caps . get_match ()) }) ; if caps . is_match () { Some (caps . clone ()) } else { None } } # [inline] fn count (self) -> usize { let CapturesMatches { re , mut cache , it , .. } = self ; let cache = & mut * cache ; it . into_half_matches_iter (| input | Ok (re . search_half_with (cache , input)) ,) . count () } }
};
}
