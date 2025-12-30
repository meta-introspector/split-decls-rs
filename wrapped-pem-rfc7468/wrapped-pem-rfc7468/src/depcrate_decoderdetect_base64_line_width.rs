// Generated macro for detect_base64_line_width (function)
macro_rules! Depcrate_decoderdetect_base64_line_width {
() => {
// Module: crate::decoder
// Provides: {"detect_base64_line_width"}
// Dependencies: {}
# [doc = " Attempt to detect the Base64 line width for the given PEM document."] # [doc = ""] # [doc = " NOTE: not constant time with respect to the input."] pub fn detect_base64_line_width (pem : & [u8]) -> Result < usize > { Ok (Encapsulation :: try_from (pem) ? . encapsulated_text_line_width ()) }
};
}
