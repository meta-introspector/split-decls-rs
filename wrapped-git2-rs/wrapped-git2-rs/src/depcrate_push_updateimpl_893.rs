// Generated macro for impl_893 (impl)
macro_rules! Depcrate_push_updateimpl_893 {
() => {
// Module: crate::push_update
// Provides: {"impl_893"}
// Dependencies: {}
impl < 'a > Binding for PushUpdate < 'a > { type Raw = * const raw :: git_push_update ; unsafe fn from_raw (raw : * const raw :: git_push_update) -> PushUpdate < 'a > { PushUpdate { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> Self :: Raw { self . raw } }
};
}
