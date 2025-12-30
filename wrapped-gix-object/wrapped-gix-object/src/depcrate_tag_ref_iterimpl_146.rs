// Generated macro for impl_146 (impl)
macro_rules! Depcrate_tag_ref_iterimpl_146 {
() => {
// Module: crate::tag::ref_iter
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a > Iterator for TagRefIter < 'a > { type Item = Result < Token < 'a > , crate :: decode :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . is_empty () { return None ; } match Self :: next_inner (self . data , & mut self . state) { Ok ((data , token)) => { self . data = data ; Some (Ok (token)) } Err (err) => { self . data = & [] ; Some (Err (err)) } } } }
};
}
