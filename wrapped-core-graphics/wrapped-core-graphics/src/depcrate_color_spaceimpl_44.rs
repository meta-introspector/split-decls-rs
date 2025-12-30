// Generated macro for impl_44 (impl)
macro_rules! Depcrate_color_spaceimpl_44 {
() => {
// Module: crate::color_space
// Provides: {"impl_44"}
// Dependencies: {}
impl CGColorSpace { pub fn type_id () -> CFTypeID { unsafe { CGColorSpaceGetTypeID () } } pub fn create_with_name (name : CFStringRef) -> Option < CGColorSpace > { unsafe { let p = CGColorSpaceCreateWithName (name) ; if ! p . is_null () { Some (CGColorSpace :: from_ptr (p)) } else { None } } } # [inline] pub fn create_device_rgb () -> CGColorSpace { unsafe { let result = CGColorSpaceCreateDeviceRGB () ; CGColorSpace :: from_ptr (result) } } # [inline] pub fn create_device_gray () -> CGColorSpace { unsafe { let result = CGColorSpaceCreateDeviceGray () ; CGColorSpace :: from_ptr (result) } } }
};
}
