// Generated macro for ID2D1Bitmap_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_return_udtID2D1Bitmap_Vtbl {
() => {
// Module: crate::interface_cpp_return_udt
// Provides: {"ID2D1Bitmap_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ID2D1Bitmap_Vtbl { pub base__ : ID2D1Image_Vtbl , pub GetSize : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut D2D_SIZE_F) , GetPixelSize : usize , GetPixelFormat : usize , pub GetDpi : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut f32 , * mut f32) , CopyFromBitmap : usize , CopyFromRenderTarget : usize , CopyFromMemory : usize , }
};
}
