// Generated macro for impl_212 (impl)
macro_rules! Depcrate_numberimpl_212 {
() => {
// Module: crate::number
// Provides: {"impl_212"}
// Dependencies: {}
impl CFNumber { # [inline] pub fn to_i32 (& self) -> Option < i32 > { unsafe { let mut value : i32 = 0 ; let ok = CFNumberGetValue (self . 0 , kCFNumberSInt32Type , & mut value as * mut i32 as * mut c_void ,) ; if ok { Some (value) } else { None } } } # [inline] pub fn to_i64 (& self) -> Option < i64 > { unsafe { let mut value : i64 = 0 ; let ok = CFNumberGetValue (self . 0 , kCFNumberSInt64Type , & mut value as * mut i64 as * mut c_void ,) ; if ok { Some (value) } else { None } } } # [inline] pub fn to_f32 (& self) -> Option < f32 > { unsafe { let mut value : f32 = 0.0 ; let ok = CFNumberGetValue (self . 0 , kCFNumberFloat32Type , & mut value as * mut f32 as * mut c_void ,) ; if ok { Some (value) } else { None } } } # [inline] pub fn to_f64 (& self) -> Option < f64 > { unsafe { let mut value : f64 = 0.0 ; let ok = CFNumberGetValue (self . 0 , kCFNumberFloat64Type , & mut value as * mut f64 as * mut c_void ,) ; if ok { Some (value) } else { None } } } }
};
}
