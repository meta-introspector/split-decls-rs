// Generated macro for impl_71 (impl)
macro_rules! Depcrate_mockimpl_71 {
() => {
// Module: crate::mock
// Provides: {"impl_71"}
// Dependencies: {}
impl Stream for Handle { type Item = Result < Frame , Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut self . codec) . poll_next (cx) } }
};
}
