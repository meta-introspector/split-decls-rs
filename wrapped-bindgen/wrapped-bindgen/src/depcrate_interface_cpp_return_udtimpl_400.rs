// Generated macro for impl_400 (impl)
macro_rules! Depcrate_interface_cpp_return_udtimpl_400 {
() => {
// Module: crate::interface_cpp_return_udt
// Provides: {"impl_400"}
// Dependencies: {}
impl ID2D1Bitmap_Vtbl { pub const fn new < Identity : ID2D1Bitmap_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetSize < Identity : ID2D1Bitmap_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , result__ : * mut D2D_SIZE_F ,) { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; * result__ = ID2D1Bitmap_Impl :: GetSize (this) } } unsafe extern "system" fn GetDpi < Identity : ID2D1Bitmap_Impl , const OFFSET : isize > (this : * mut core :: ffi :: c_void , dpix : * mut f32 , dpiy : * mut f32 ,) { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; ID2D1Bitmap_Impl :: GetDpi (this , core :: mem :: transmute_copy (& dpix) , core :: mem :: transmute_copy (& dpiy) ,) } } Self { base__ : ID2D1Image_Vtbl :: new :: < Identity , OFFSET > () , GetSize : GetSize :: < Identity , OFFSET > , GetPixelSize : 0 , GetPixelFormat : 0 , GetDpi : GetDpi :: < Identity , OFFSET > , CopyFromBitmap : 0 , CopyFromRenderTarget : 0 , CopyFromMemory : 0 , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID2D1Bitmap as windows_core :: Interface > :: IID || iid == & < ID2D1Resource as windows_core :: Interface > :: IID || iid == & < ID2D1Image as windows_core :: Interface > :: IID } }
};
}
