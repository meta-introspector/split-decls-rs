// Generated macro for delegate_stream (macro)
macro_rules! Depcratedelegate_stream {
() => {
// Module: crate
// Provides: {"delegate_stream"}
// Dependencies: {}
macro_rules ! delegate_stream { ($ field : ident) => { fn poll_next (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < Option < Self :: Item >> { self . project () .$ field . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self .$ field . size_hint () } } ; }
};
}
