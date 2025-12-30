// Generated macro for impl_66 (impl)
macro_rules! Depcrate_commit_ref_iterimpl_66 {
() => {
// Module: crate::commit::ref_iter
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > Iterator for CommitRefIterRaw < 'a > { type Item = Result < RawToken < 'a > , crate :: decode :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . is_empty () { return None ; } match CommitRefIter :: next_inner (self . data , & mut self . state) { Ok ((remaining , token)) => { let consumed = self . data . len () - remaining . len () ; let start = self . offset ; let end = start + consumed ; self . offset = end ; self . data = remaining ; Some (Ok (RawToken { token , token_range : start .. end , })) } Err (err) => { self . data = & [] ; Some (Err (err)) } } } }
};
}
