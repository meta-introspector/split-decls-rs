// Generated macro for write_trim_password (function)
macro_rules! Depcrate_mask_passwordwrite_trim_password {
() => {
// Module: crate::mask_password
// Provides: {"write_trim_password"}
// Dependencies: {}
# [doc = " Writes an IRI with the password part trimmed."] fn write_trim_password (f : & mut fmt :: Formatter < '_ > , s : & str , pw_range : Range < usize >) -> fmt :: Result { write_with_masked_password (f , s , pw_range , "") }
};
}
