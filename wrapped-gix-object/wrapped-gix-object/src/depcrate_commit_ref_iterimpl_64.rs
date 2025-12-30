// Generated macro for impl_64 (impl)
macro_rules! Depcrate_commit_ref_iterimpl_64 {
() => {
// Module: crate::commit::ref_iter
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > Iterator for CommitRefIter < 'a > { type Item = Result < Token < 'a > , crate :: decode :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . is_empty () { return None ; } match Self :: next_inner (self . data , & mut self . state) { Ok ((data , token)) => { self . data = data ; Some (Ok (token)) } Err (err) => { self . data = & [] ; Some (Err (err)) } } } }
};
}
