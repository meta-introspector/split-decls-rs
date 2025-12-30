// Generated macro for impl_1188 (impl)
macro_rules! Depcrate_base_rkyv_wrappersimpl_1188 {
() => {
// Module: crate::base::rkyv_wrappers
// Provides: {"impl_1188"}
// Dependencies: {}
impl < OT : ? Sized , NT : ? Sized > ArchiveWith < PhantomData < OT > > for CustomPhantom < NT > { type Archived = PhantomData < NT > ; type Resolver = () ; # [inline] unsafe fn resolve_with (_ : & PhantomData < OT > , _ : usize , _ : Self :: Resolver , _ : * mut Self :: Archived ,) { } }
};
}
