// Generated macro for impl_173 (impl)
macro_rules! Depcrate_nonblockimpl_173 {
() => {
// Module: crate::nonblock
// Provides: {"impl_173"}
// Dependencies: {}
impl Connection { fn filters_mut (& self) -> std :: cell :: RefMut < Filters < FilterCb > > { self . filters . borrow_mut () } fn replies_mut (& self) -> std :: cell :: RefMut < Replies < RepliesCb > > { self . replies . borrow_mut () } }
};
}
