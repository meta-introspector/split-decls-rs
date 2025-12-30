// Generated macro for Class (struct)
macro_rules! Depcrate_ffiClass {
() => {
// Module: crate::ffi
// Provides: {"Class"}
// Dependencies: {}
# [doc = " Type for block class ISAs."] # [doc = ""] # [doc = " This will likely become an extern type in the future."] # [repr (C)] # [allow (missing_debug_implementations)] pub struct Class { # [doc = " The size probably doesn't really matter here, as we only ever use the"] # [doc = " classes behind pointers, but let's import it with the correct size to"] # [doc = " be sure."] # [doc = ""] # [doc = " This applies with the compiler-rt runtime and with Apple's runtime."] # [cfg (not (any (feature = "gnustep-1-7" , feature = "unstable-objfw")))] _priv : [* mut c_void ; 32] , # [doc = " The size of this is unknown, so let's use a ZST so the compiler"] # [doc = " doesn't assume anything about the size."] # [cfg (any (feature = "gnustep-1-7" , feature = "unstable-objfw"))] _priv : [u8 ; 0] , # [doc = " Mark as `!Send + !Sync + !Unpin` and as mutable behind shared"] # [doc = " references (`!Freeze`)."] # [doc = ""] # [doc = " Same as `objc2::ffi::OpaqueData`."] _opaque : UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > , }
};
}
