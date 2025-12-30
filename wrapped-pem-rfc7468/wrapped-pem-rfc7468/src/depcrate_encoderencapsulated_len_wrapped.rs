// Generated macro for encapsulated_len_wrapped (function)
macro_rules! Depcrate_encoderencapsulated_len_wrapped {
() => {
// Module: crate::encoder
// Provides: {"encapsulated_len_wrapped"}
// Dependencies: {}
# [doc = " Compute the length of a PEM encoded document with the Base64 body"] # [doc = " line wrapped at the specified `width`."] # [doc = ""] # [doc = " This is the same as [`encapsulated_len`], which defaults to a width of 64."] # [doc = ""] # [doc = " Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides"] # [doc = " 64 is technically non-compliant:"] # [doc = ""] # [doc = " > Generators MUST wrap the base64-encoded lines so that each line"] # [doc = " > consists of exactly 64 characters except for the final line, which"] # [doc = " > will encode the remainder of the data (within the 64-character line"] # [doc = " > boundary)"] # [doc = ""] # [doc = " [RFC7468 § 2]: https://datatracker.ietf.org/doc/html/rfc7468#section-2"] pub fn encapsulated_len_wrapped (label : & str , line_width : usize , line_ending : LineEnding , input_len : usize ,) -> Result < usize > { if line_width < 4 { return Err (Error :: Length) ; } let base64_len = input_len . checked_mul (4) . and_then (| n | n . checked_div (3)) . and_then (| n | n . checked_add (3)) . ok_or (Error :: Length) ? & ! 3 ; let base64_len_wrapped = base64_len_wrapped (base64_len , line_width , line_ending) ? ; encapsulated_len_inner (label , line_ending , base64_len_wrapped) }
};
}
