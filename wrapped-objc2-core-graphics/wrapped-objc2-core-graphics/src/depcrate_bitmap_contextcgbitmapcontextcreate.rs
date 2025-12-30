// Generated macro for CGBitmapContextCreate (function)
macro_rules! Depcrate_bitmap_contextCGBitmapContextCreate {
() => {
// Module: crate::bitmap_context
// Provides: {"CGBitmapContextCreate"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " `data` must be a valid pointer or null."] # [cfg (all (feature = "CGColorSpace" , feature = "CGContext" , feature = "CGImage"))] # [inline] pub unsafe extern "C-unwind" fn CGBitmapContextCreate (data : * mut c_void , width : usize , height : usize , bits_per_component : usize , bytes_per_row : usize , space : Option < & CGColorSpace > , bitmap_info : u32 ,) -> Option < CFRetained < CGContext > > { extern "C-unwind" { fn CGBitmapContextCreate (data : * mut c_void , width : usize , height : usize , bits_per_component : usize , bytes_per_row : usize , space : Option < & CGColorSpace > , bitmap_info : u32 ,) -> Option < NonNull < CGContext > > ; } let ret = unsafe { CGBitmapContextCreate (data , width , height , bits_per_component , bytes_per_row , space , bitmap_info ,) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
