// Generated macro for impl_750 (impl)
macro_rules! Depcrate_frameimpl_750 {
() => {
// Module: crate::frame
// Provides: {"impl_750"}
// Dependencies: {}
impl Iterator for Iter { type Item = Result < Frame , InvalidFrame > ; fn next (& mut self) -> Option < Self :: Item > { if ! self . bytes . has_remaining () { return None ; } match self . try_next () { Ok (x) => Some (Ok (x)) , Err (e) => { self . bytes . clear () ; Some (Err (InvalidFrame { ty : self . last_ty , reason : e . reason () , })) } } } }
};
}
