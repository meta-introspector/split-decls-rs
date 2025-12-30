// Generated macro for append_quoted_key_value (function)
macro_rules! Depcrate_digestappend_quoted_key_value {
() => {
// Module: crate::digest
// Provides: {"append_quoted_key_value"}
// Dependencies: {}
# [inline (never)] fn append_quoted_key_value (out : & mut String , key : & str , value : & str) -> Result < () , String > { out . push_str (key) ; out . push_str ("=\"") ; let mut first_unwritten = 0 ; let bytes = value . as_bytes () ; for (i , & b) in bytes . iter () . enumerate () { let class = char_classes (b) ; if (class & C_QDTEXT) != 0 { } else if (class & C_ESCAPABLE) != 0 { out . push_str (& value [first_unwritten .. i]) ; out . push ('\\') ; out . push (char :: from (b)) ; first_unwritten = i + 1 ; } else { return Err (format ! ("invalid {} value {:?}" , key , value)) ; } } out . push_str (& value [first_unwritten ..]) ; out . push_str ("\", ") ; Ok (()) }
};
}
