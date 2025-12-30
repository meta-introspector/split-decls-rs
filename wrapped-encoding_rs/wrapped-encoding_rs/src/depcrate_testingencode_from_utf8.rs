// Generated macro for encode_from_utf8 (function)
macro_rules! Depcrate_testingencode_from_utf8 {
() => {
// Module: crate::testing
// Provides: {"encode_from_utf8"}
// Dependencies: {}
pub fn encode_from_utf8 (encoding : & 'static Encoding , string : & str , expect : & [u8]) { let mut encoder = encoding . new_encoder () ; let mut dest : Vec < u8 > = Vec :: with_capacity (10 * (string . len () + 1)) ; let capacity = dest . capacity () ; dest . resize (capacity , 0u8) ; let (complete , read , written , _) = encoder . encode_from_utf8 (string , & mut dest , true) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } assert_eq ! (read , string . len ()) ; assert_eq ! (written , expect . len ()) ; dest . truncate (written) ; assert_eq ! (& dest [..] , expect) ; }
};
}
