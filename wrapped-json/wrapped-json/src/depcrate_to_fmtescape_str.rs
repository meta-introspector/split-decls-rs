// Generated macro for escape_str (function)
macro_rules! Depcrate_to_fmtescape_str {
() => {
// Module: crate::to_fmt
// Provides: {"escape_str"}
// Dependencies: {}
# [inline (always)] fn escape_str (value : & str , mut out : impl Write) -> Result < () , fmt :: Error > { let bytes = value . as_bytes () ; let mut start = 0 ; for (i , & byte) in bytes . iter () . enumerate () { let escape = ESCAPE [byte as usize] ; if escape == 0 { continue ; } if start < i { _try_no_conv ! (out . write_str (& value [start .. i])) ; } match escape { BB => _try_no_conv ! (out . write_str ("\\b")) , TT => _try_no_conv ! (out . write_str ("\\t")) , NN => _try_no_conv ! (out . write_str ("\\n")) , FF => _try_no_conv ! (out . write_str ("\\f")) , RR => _try_no_conv ! (out . write_str ("\\r")) , QU => _try_no_conv ! (out . write_str ("\\\"")) , BS => _try_no_conv ! (out . write_str ("\\\\")) , U => { static HEX_DIGITS : [u8 ; 16] = * b"0123456789abcdef" ; _try_no_conv ! (out . write_str ("\\u00")) ; _try_no_conv ! (out . write_char (HEX_DIGITS [(byte >> 4) as usize] as char)) ; _try_no_conv ! (out . write_char (HEX_DIGITS [(byte & 0xF) as usize] as char)) ; } _ => unreachable ! () , } start = i + 1 ; } if start != bytes . len () { _try_no_conv ! (out . write_str (& value [start ..])) ; } Ok (()) }
};
}
