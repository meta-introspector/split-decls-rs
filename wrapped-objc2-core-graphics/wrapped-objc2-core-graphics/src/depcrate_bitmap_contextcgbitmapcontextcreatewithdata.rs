// Generated macro for CGBitmapContextCreateWithData (function)
macro_rules! Depcrate_bitmap_contextCGBitmapContextCreateWithData {
() => {
// Module: crate::bitmap_context
// Provides: {"CGBitmapContextCreateWithData"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `data` must be a valid pointer or null."] # [doc = " - `release_callback` must be implemented correctly."] # [doc = " - `release_info` must be a valid pointer or null."] # [cfg (all (feature = "CGColorSpace" , feature = "CGContext" , feature = "CGImage"))] # [inline] pub unsafe extern "C-unwind" fn CGBitmapContextCreateWithData (data : * mut c_void , width : usize , height : usize , bits_per_component : usize , bytes_per_row : usize , space : Option < & CGColorSpace > , bitmap_info : u32 , release_callback : CGBitmapContextReleaseDataCallback , release_info : * mut c_void ,) -> Option < CFRetained < CGContext > > { extern "C-unwind" { fn CGBitmapContextCreateWithData (data : * mut c_void , width : usize , height : usize , bits_per_component : usize , bytes_per_row : usize , space : Option < & CGColorSpace > , bitmap_info : u32 , release_callback : CGBitmapContextReleaseDataCallback , release_info : * mut c_void ,) -> Option < NonNull < CGContext > > ; } let ret = unsafe { CGBitmapContextCreateWithData (data , width , height , bits_per_component , bytes_per_row , space , bitmap_info , release_callback , release_info ,) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
