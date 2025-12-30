// Generated macro for validate_raw_str (function)
macro_rules! Depcratevalidate_raw_str {
() => {
// Module: crate
// Provides: {"validate_raw_str"}
// Dependencies: {}
# [doc = " Validates a raw string literal. Used for getting more information about a"] # [doc = " problem with a `RawStr`/`RawByteStr` with a `None` field."] # [inline] pub fn validate_raw_str (input : & str , prefix_len : u32) -> Result < () , RawStrError > { debug_assert ! (! input . is_empty ()) ; let mut cursor = Cursor :: new (input , FrontmatterAllowed :: No) ; for _ in 0 .. prefix_len { cursor . bump () . unwrap () ; } cursor . raw_double_quoted_string (prefix_len) . map (| _ | ()) }
};
}
