macro_rules! deps {
    () => {
        Result!();
        SmallCString!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl SmallCString { # [inline] pub fn new (s : & str) -> Result < Self , NulError > { if s . as_bytes () . contains (& 0_u8) { return Err (Self :: fabricate_nul_error (s)) ; } let mut buf = SmallVec :: with_capacity (s . len () + 1) ; buf . extend_from_slice (s . as_bytes ()) ; buf . push (0) ; let res = Self (buf) ; res . debug_checks () ; Ok (res) } # [inline] pub fn as_str (& self) -> & str { self . debug_checks () ; unsafe { std :: str :: from_utf8_unchecked (self . as_bytes_without_nul ()) } } # [doc = " Get the bytes not including the NUL terminator. E.g. the bytes which"] # [doc = " make up our `str`:"] # [doc = " - `SmallCString::new(\"foo\").as_bytes_without_nul() == b\"foo\"`"] # [doc = " - `SmallCString::new(\"foo\").as_bytes_with_nul() == b\"foo\\0\"`"] # [inline] pub fn as_bytes_without_nul (& self) -> & [u8] { self . debug_checks () ; & self . 0 [.. self . len ()] } # [doc = " Get the bytes behind this str *including* the NUL terminator. This"] # [doc = " should never return an empty slice."] # [inline] pub fn as_bytes_with_nul (& self) -> & [u8] { self . debug_checks () ; & self . 0 } # [inline] # [cfg (debug_assertions)] fn debug_checks (& self) { debug_assert_ne ! (self . 0 . len () , 0) ; debug_assert_eq ! (self . 0 [self . 0 . len () - 1] , 0) ; let strbytes = & self . 0 [.. (self . 0 . len () - 1)] ; debug_assert ! (! strbytes . contains (& 0)) ; debug_assert ! (std :: str :: from_utf8 (strbytes) . is_ok ()) ; } # [inline] # [cfg (not (debug_assertions))] fn debug_checks (& self) { } # [inline] pub fn len (& self) -> usize { debug_assert_ne ! (self . 0 . len () , 0) ; self . 0 . len () - 1 } # [inline] # [allow (unused)] pub fn is_empty (& self) -> bool { self . len () == 0 } # [inline] pub fn as_cstr (& self) -> & CStr { let bytes = self . as_bytes_with_nul () ; debug_assert ! (CStr :: from_bytes_with_nul (bytes) . is_ok ()) ; unsafe { CStr :: from_bytes_with_nul_unchecked (bytes) } } # [cold] fn fabricate_nul_error (b : & str) -> NulError { CString :: new (b) . unwrap_err () } }
    };
}

impl_663!()