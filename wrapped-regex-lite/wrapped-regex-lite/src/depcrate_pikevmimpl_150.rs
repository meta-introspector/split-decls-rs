// Generated macro for impl_150 (impl)
macro_rules! Depcrate_pikevmimpl_150 {
() => {
// Module: crate::pikevm
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'r , 'h > FindMatches < 'r , 'h > { # [doc = " Handles the special case of an empty match by ensuring that 1) the"] # [doc = " iterator always advances and 2) empty matches never overlap with other"] # [doc = " matches."] # [doc = ""] # [doc = " Note that we mark this cold and forcefully prevent inlining because"] # [doc = " handling empty matches like this is extremely rare and does require a"] # [doc = " bit of code, comparatively. Keeping this code out of the main iterator"] # [doc = " function keeps it smaller and more amenable to inlining itself."] # [cold] # [inline (never)] fn handle_overlapping_empty_match (& mut self , mut m : (usize , usize) ,) -> Option < (usize , usize) > { assert ! (m . 0 >= m . 1) ; if Some (m . 1) == self . last_match_end { let len = core :: cmp :: max (1 , utf8 :: decode (& self . haystack [self . at ..]) . 1) ; self . at = self . at . checked_add (len) . unwrap () ; if ! self . pikevm . search (& mut self . cache , self . haystack , self . at , self . haystack . len () , false , & mut self . slots ,) { return None ; } m = (self . slots [0] . unwrap () . get () , self . slots [1] . unwrap () . get ()) ; } Some (m) } }
};
}
