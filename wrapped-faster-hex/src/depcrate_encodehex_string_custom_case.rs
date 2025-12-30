// Generated macro for hex_string_custom_case (function)
macro_rules! Depcrate_encodehex_string_custom_case {
() => {
// Module: crate::encode
// Provides: {"hex_string_custom_case"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] fn hex_string_custom_case < const N : usize > (src : & [u8] , upper_case : bool) -> String < N > { let mut buffer = Vec :: < _ , N > :: new () ; buffer . resize (src . len () * 2 , 0) . expect ("String<N> capacity too short") ; if upper_case { hex_encode_upper (src , & mut buffer) . expect ("hex_string") ; } else { hex_encode (src , & mut buffer) . expect ("hex_string") ; } if cfg ! (debug_assertions) { String :: from_utf8 (buffer) . unwrap () } else { unsafe { String :: from_utf8_unchecked (buffer) } } }
};
}
