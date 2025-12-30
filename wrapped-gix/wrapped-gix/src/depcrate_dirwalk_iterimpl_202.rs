// Generated macro for impl_202 (impl)
macro_rules! Depcrate_dirwalk_iterimpl_202 {
() => {
// Module: crate::dirwalk::iter
// Provides: {"impl_202"}
// Dependencies: {}
impl Iterator for Iter { type Item = Result < Item , dirwalk :: Error > ; fn next (& mut self) -> Option < Self :: Item > { # [cfg (feature = "parallel")] { let (rx , _join) = self . rx_and_join . as_ref () ? ; match rx . recv () . ok () { Some (item) => Some (Ok (item)) , None => { let (_rx , handle) = self . rx_and_join . take () ? ; match handle . join () . expect ("no panic") { Ok (out) => { self . out = Some (out) ; None } Err (err) => Some (Err (err)) , } } } } # [cfg (not (feature = "parallel"))] self . items . next () . map (Ok) } }
};
}
