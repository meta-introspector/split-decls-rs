macro_rules! expect_optional_utf8 {
    () => {
        unsafe fn expect_optional_utf8 < 'a > (p_str : * const c_char , description : & 'static str ,) -> Option < & 'a str > { if p_str . is_null () { return None ; } CStr :: from_ptr (p_str) . to_str () . unwrap_or_else (| _ | panic ! ("received non-utf8 string as {description}")) . into () }
    };
}

expect_optional_utf8!()