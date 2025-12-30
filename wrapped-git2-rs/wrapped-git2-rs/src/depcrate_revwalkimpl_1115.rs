// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_revwalkimpl_1115 {
() => {
// Module: crate::revwalk
// Provides: {"impl_1115"}
// Dependencies: {}
impl < 'repo , 'cb , C : FnMut (Oid) -> bool > Iterator for RevwalkWithHideCb < 'repo , 'cb , C > { type Item = Result < Oid , Error > ; fn next (& mut self) -> Option < Result < Oid , Error > > { let out = self . revwalk . next () ; crate :: panic :: check () ; out } }
};
}
