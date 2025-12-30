// Generated macro for Opaque (struct)
macro_rules! Depcrate_opaqueOpaque {
() => {
// Module: crate::opaque
// Provides: {"Opaque"}
// Dependencies: {}
# [doc = " An opaque type."] # [doc = ""] # [doc = " This is used to avoid problems with e.g. getting references from"] # [doc = " `CFArray` (we can't use `c_void` as the default type, as `&c_void` would"] # [doc = " be incorrect)."] # [repr (C)] # [doc (hidden)] # [allow (dead_code , unreachable_pub)] pub struct Opaque { inner : [u8 ; 0] , _p : UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > , }
};
}
