// Generated macro for base64_len_wrapped (function)
macro_rules! Depcrate_encoderbase64_len_wrapped {
() => {
// Module: crate::encoder
// Provides: {"base64_len_wrapped"}
// Dependencies: {}
# [doc = " Compute Base64 length line-wrapped at the specified width with the given"] # [doc = " line ending."] fn base64_len_wrapped (base64_len : usize , line_width : usize , line_ending : LineEnding ,) -> Result < usize > { base64_len . saturating_sub (1) . checked_div (line_width) . and_then (| lines | lines . checked_mul (line_ending . len ())) . and_then (| len | len . checked_add (base64_len)) . ok_or (Error :: Length) }
};
}
