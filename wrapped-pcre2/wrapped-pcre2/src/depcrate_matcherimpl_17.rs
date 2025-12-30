// Generated macro for impl_17 (impl)
macro_rules! Depcrate_matcherimpl_17 {
() => {
// Module: crate::matcher
// Provides: {"impl_17"}
// Dependencies: {}
impl Matcher for RegexMatcher { type Captures = RegexCaptures ; type Error = Error ; fn find_at (& self , haystack : & [u8] , at : usize ,) -> Result < Option < Match > , Error > { Ok (self . regex . find_at (haystack , at) . map_err (Error :: regex) ? . map (| m | Match :: new (m . start () , m . end ()))) } fn new_captures (& self) -> Result < RegexCaptures , Error > { Ok (RegexCaptures :: new (self . regex . capture_locations ())) } fn capture_count (& self) -> usize { self . regex . captures_len () } fn capture_index (& self , name : & str) -> Option < usize > { self . names . get (name) . map (| i | * i) } fn try_find_iter < F , E > (& self , haystack : & [u8] , mut matched : F ,) -> Result < Result < () , E > , Error > where F : FnMut (Match) -> Result < bool , E > , { for result in self . regex . find_iter (haystack) { let m = result . map_err (Error :: regex) ? ; match matched (Match :: new (m . start () , m . end ())) { Ok (true) => continue , Ok (false) => return Ok (Ok (())) , Err (err) => return Ok (Err (err)) , } } Ok (Ok (())) } fn captures_at (& self , haystack : & [u8] , at : usize , caps : & mut RegexCaptures ,) -> Result < bool , Error > { Ok (self . regex . captures_read_at (& mut caps . locs , haystack , at) . map_err (Error :: regex) ? . is_some ()) } }
};
}
