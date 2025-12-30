// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl IOSurface { pub fn get_id (& self) -> IOSurfaceID { unsafe { IOSurfaceGetID (self . as_concrete_TypeRef ()) } } # [doc = " Binds to the current GL texture."] pub fn bind_to_gl_texture (& self , width : i32 , height : i32 , has_alpha : bool) { unsafe { let context = CGLGetCurrentContext () ; let gl_error = CGLTexImageIOSurface2D (context , TEXTURE_RECTANGLE_ARB , if has_alpha { RGBA as GLenum } else { RGB as GLenum } , width , height , BGRA as GLenum , UNSIGNED_INT_8_8_8_8_REV , self . as_concrete_TypeRef () as * mut c_void , 0 ,) ; if gl_error != kCGLNoError { let error_msg = CStr :: from_ptr (CGLErrorString (gl_error)) ; let error_msg = error_msg . to_string_lossy () ; panic ! ("{}" , error_msg . leak ()) ; } } } pub fn upload (& self , data : & [u8]) { unsafe { let surface = self . as_concrete_TypeRef () ; let mut seed = 0 ; IOSurfaceLock (surface , 0 , & mut seed) ; let height = IOSurfaceGetHeight (surface) ; let stride = IOSurfaceGetBytesPerRow (surface) ; let size = height * stride ; let address = IOSurfaceGetBaseAddress (surface) as * mut u8 ; let dest : & mut [u8] = slice :: from_raw_parts_mut (address , size) ; dest . clone_from_slice (data) ; IOSurfaceUnlock (surface , 0 , & mut seed) ; } } }
};
}
