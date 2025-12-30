// Generated macro for name_len (function)
macro_rules! Depcrate_utilsname_len {
() => {
// Module: crate::utils
// Provides: {"name_len"}
// Dependencies: {}
# [doc = " Calculates name from an element-like content. Name is the first word in `content`,"] # [doc = " where word boundaries is XML whitespace characters."] # [doc = ""] # [doc = " 'Whitespace' refers to the definition used by [`is_whitespace`]."] # [inline] pub const fn name_len (mut bytes : & [u8]) -> usize { let mut len = 0 ; while let [first , rest @ ..] = bytes { if is_whitespace (* first) { break ; } len += 1 ; bytes = rest ; } len }
};
}
