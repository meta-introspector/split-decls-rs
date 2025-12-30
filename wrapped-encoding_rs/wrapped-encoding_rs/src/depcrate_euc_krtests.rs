// Generated macro for tests (module)
macro_rules! Depcrate_euc_krtests {
() => {
// Module: crate::euc_kr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "alloc"))] mod tests { use super :: super :: testing :: * ; use super :: super :: * ; fn decode_euc_kr (bytes : & [u8] , expect : & str) { decode (EUC_KR , bytes , expect) ; } fn encode_euc_kr (string : & str , expect : & [u8]) { encode (EUC_KR , string , expect) ; } # [test] fn test_euc_kr_decode () { decode_euc_kr (b"" , & "") ; decode_euc_kr (b"\x61\x62" , "\u{0061}\u{0062}") ; decode_euc_kr (b"\x81\x41" , "\u{AC02}") ; decode_euc_kr (b"\x81\x5B" , "\u{FFFD}\x5B") ; decode_euc_kr (b"\xFD\xFE" , "\u{8A70}") ; decode_euc_kr (b"\xFE\x41" , "\u{FFFD}\x41") ; decode_euc_kr (b"\xFF\x41" , "\u{FFFD}\x41") ; decode_euc_kr (b"\x80\x41" , "\u{FFFD}\x41") ; decode_euc_kr (b"\xA1\xFF" , "\u{FFFD}") ; decode_euc_kr (b"\x81\xFF" , "\u{FFFD}") ; } # [test] fn test_euc_kr_encode () { encode_euc_kr ("" , b"") ; encode_euc_kr ("\u{0061}\u{0062}" , b"\x61\x62") ; encode_euc_kr ("\u{AC02}" , b"\x81\x41") ; encode_euc_kr ("\u{8A70}" , b"\xFD\xFE") ; } # [test] fn test_euc_kr_encode_from_two_low_surrogates () { let expectation = b"&#65533;&#65533;" ; let mut output = [0u8 ; 40] ; let mut encoder = EUC_KR . new_encoder () ; let (result , read , written , had_errors) = encoder . encode_from_utf16 (& [0xDC00u16 , 0xDEDEu16] , & mut output [..] , true) ; assert_eq ! (result , CoderResult :: InputEmpty) ; assert_eq ! (read , 2) ; assert_eq ! (written , expectation . len ()) ; assert ! (had_errors) ; assert_eq ! (& output [.. written] , expectation) ; } }
};
}
