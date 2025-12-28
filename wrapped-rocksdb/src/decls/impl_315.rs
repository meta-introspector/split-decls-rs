macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl PropertyName { # [doc = " Creates a new object from valid nul-terminated UTF-8 string. The string"] # [doc = " must not contain interior nul bytes."] # [inline] unsafe fn from_vec_with_nul_unchecked (inner : Vec < u8 >) -> Self { Self (unsafe { CString :: from_vec_with_nul_unchecked (inner) }) } # [doc = " Converts the value into a C string."] # [inline] pub fn into_c_string (self) -> CString { self . 0 } # [doc = " Converts the property name into a string."] # [doc = ""] # [doc = " Nul byte terminating the underlying C string is not included in the"] # [doc = " returned value."] # [inline] pub fn into_string (self) -> String { unsafe { String :: from_utf8_unchecked (self . 0 . into_bytes ()) } } }
    };
}

impl_315!()