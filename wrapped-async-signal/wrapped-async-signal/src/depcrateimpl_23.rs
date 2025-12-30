// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Stream for Signals { type Item = io :: Result < Signal > ; # [inline] fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut & * self) . poll_next (cx) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
