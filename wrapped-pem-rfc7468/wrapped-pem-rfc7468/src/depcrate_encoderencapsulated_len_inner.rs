// Generated macro for encapsulated_len_inner (function)
macro_rules! Depcrate_encoderencapsulated_len_inner {
() => {
// Module: crate::encoder
// Provides: {"encapsulated_len_inner"}
// Dependencies: {}
# [doc = " Compute the encapsulated length of Base64 data of the given length."] fn encapsulated_len_inner (label : & str , line_ending : LineEnding , base64_len : usize ,) -> Result < usize > { [PRE_ENCAPSULATION_BOUNDARY . len () , label . len () , ENCAPSULATION_BOUNDARY_DELIMITER . len () , line_ending . len () , base64_len , line_ending . len () , POST_ENCAPSULATION_BOUNDARY . len () , label . len () , ENCAPSULATION_BOUNDARY_DELIMITER . len () , line_ending . len () ,] . into_iter () . try_fold (0usize , | acc , len | acc . checked_add (len)) . ok_or (Error :: Length) }
};
}
