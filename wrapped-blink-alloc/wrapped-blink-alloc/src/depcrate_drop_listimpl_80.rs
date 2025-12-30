// Generated macro for impl_80 (impl)
macro_rules! Depcrate_drop_listimpl_80 {
() => {
// Module: crate::drop_list
// Provides: {"impl_80"}
// Dependencies: {}
impl Drops { unsafe fn drop (ptr : NonNull < Self >) -> Option < NonNull < Self > > { let Drops { count , drop , next } = * ptr . as_ref () ; unsafe { (drop) (ptr , count) } ; next } }
};
}
