// Generated macro for impl_174 (impl)
macro_rules! Depcrate_nonblockimpl_174 {
() => {
// Module: crate::nonblock
// Provides: {"impl_174"}
// Dependencies: {}
impl LocalConnection { fn filters_mut (& self) -> std :: cell :: RefMut < Filters < LocalFilterCb > > { self . filters . borrow_mut () } fn replies_mut (& self) -> std :: cell :: RefMut < Replies < LocalRepliesCb > > { self . replies . borrow_mut () } }
};
}
