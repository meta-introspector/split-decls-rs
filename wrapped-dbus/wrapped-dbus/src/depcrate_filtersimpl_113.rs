// Generated macro for impl_113 (impl)
macro_rules! Depcrate_filtersimpl_113 {
() => {
// Module: crate::filters
// Provides: {"impl_113"}
// Dependencies: {}
impl < F > Filters < F > { pub fn add (& mut self , m : MatchRule < 'static > , f : F) -> Token { let id = self . nextid ; self . nextid . 0 += 1 ; self . list . insert (id , (m , f)) ; id } pub fn insert (& mut self , (t , m , f) : (Token , MatchRule < 'static > , F)) { self . list . insert (t , (m , f)) ; } pub fn remove (& mut self , id : Token) -> Option < (MatchRule < 'static > , F) > { self . list . remove (& id) } # [doc = " Removes and returns the first filter which matches the given message."] pub fn remove_first_matching (& mut self , msg : & Message) -> Option < (Token , MatchRule < 'static > , F) > { if let Some (k) = self . list . iter_mut () . find_map (| (k , v) | if v . 0 . matches (& msg) { Some (* k) } else { None }) { let v = self . list . remove (& k) . unwrap () ; Some ((k , v . 0 , v . 1)) } else { None } } # [doc = " Removes and returns all filters which match the given message."] pub fn remove_all_matching (& mut self , msg : & Message) -> Vec < (Token , MatchRule < 'static > , F) > { let matching : Vec < _ > = self . list . iter () . filter_map (| (k , v) | if v . 0 . matches (& msg) { Some (* k) } else { None }) . collect () ; matching . into_iter () . map (| k | { let v = self . list . remove (& k) . unwrap () ; (k , v . 0 , v . 1) }) . collect () } }
};
}
