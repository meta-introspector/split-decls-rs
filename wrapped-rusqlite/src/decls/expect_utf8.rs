macro_rules! expect_utf8 {
    () => {
        unsafe fn expect_utf8 < 'a > (p_str : * const c_char , description : & 'static str) -> & 'a str { expect_optional_utf8 (p_str , description) . unwrap_or_else (| | panic ! ("received empty {description}")) }
    };
}

expect_utf8!()