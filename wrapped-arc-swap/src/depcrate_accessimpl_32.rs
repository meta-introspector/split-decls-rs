// Generated macro for impl_32 (impl)
macro_rules! Depcrate_accessimpl_32 {
() => {
// Module: crate::access
// Provides: {"impl_32"}
// Dependencies: {}
impl < A , F , T , R > Access < R > for Map < A , T , F > where A : Access < T > , F : Fn (& T) -> & R + Clone , { type Guard = MapGuard < A :: Guard , F , T , R > ; fn load (& self) -> Self :: Guard { let guard = self . access . load () ; MapGuard { guard , projection : self . projection . clone () , _t : PhantomData , } } }
};
}
