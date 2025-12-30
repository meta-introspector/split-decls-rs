// Generated macro for impl_8161 (impl)
macro_rules! Depcrate_non_copy_constimpl_8161 {
() => {
// Module: crate::non_copy_const
// Provides: {"impl_8161"}
// Dependencies: {}
impl BorrowCause { fn note (self) -> Option < & 'static str > { match self { Self :: Borrow => None , Self :: Deref => Some ("this deref expression is a call to `Deref::deref`") , Self :: Index => Some ("this index expression is a call to `Index::index`") , Self :: AutoDeref => Some ("there is a compiler inserted call to `Deref::deref` here") , Self :: AutoBorrow => Some ("there is a compiler inserted borrow here") , Self :: AutoDerefField => { Some ("there is a compiler inserted call to `Deref::deref` when accessing this field") } , } } }
};
}
