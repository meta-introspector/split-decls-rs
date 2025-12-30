// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl Stream for & Signals { type Item = io :: Result < Signal > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let signal = ready ! (self . notifier . poll_next (cx)) ? ; Poll :: Ready (Some (Ok (signal))) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
