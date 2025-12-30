// Generated macro for tests (module)
macro_rules! Depcrate_encodingtests {
() => {
// Module: crate::encoding
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { hex_decode , hex_encode } ; # [test] fn test_hex_decode () { for (text , expected) in [("" , Some (vec ! [])) , ("00" , Some (vec ! [0])) , ("0" , None) , ("12-0" , None) , ("120-" , None) , ("ab" , Some (vec ! [0xAB])) , ("AB" , Some (vec ! [0xAB])) , ("ABCD" , Some (vec ! [0xAB , 0xCD])) ,] { assert_eq ! (hex_decode (text) , expected) ; } } # [test] fn test_hex_encode () { for (input , expected) in [(& [] [..] , "") , (& [0] [..] , "00") , (& [0xAB] [..] , "AB") , (& [0xAB , 0xCD] [..] , "ABCD") , (& [0x12 , 0x34 , 0x56] [..] , "123456") ,] { assert_eq ! (hex_encode (input) , expected) ; } } }
};
}
