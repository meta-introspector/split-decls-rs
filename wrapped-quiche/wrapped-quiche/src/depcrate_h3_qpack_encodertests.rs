// Generated macro for tests (module)
macro_rules! Depcrate_h3_qpack_encodertests {
() => {
// Module: crate::h3::qpack::encoder
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn encode_int1 () { let expected = [0b01010] ; let mut encoded = [0 ; 1] ; let mut b = octets :: OctetsMut :: with_slice (& mut encoded) ; assert ! (encode_int (10 , 0 , 5 , & mut b) . is_ok ()) ; assert_eq ! (expected , encoded) ; } # [test] fn encode_int2 () { let expected = [0b11111 , 0b10011010 , 0b00001010] ; let mut encoded = [0 ; 3] ; let mut b = octets :: OctetsMut :: with_slice (& mut encoded) ; assert ! (encode_int (1337 , 0 , 5 , & mut b) . is_ok ()) ; assert_eq ! (expected , encoded) ; } # [test] fn encode_int3 () { let expected = [0b101010] ; let mut encoded = [0 ; 1] ; let mut b = octets :: OctetsMut :: with_slice (& mut encoded) ; assert ! (encode_int (42 , 0 , 8 , & mut b) . is_ok ()) ; assert_eq ! (expected , encoded) ; } # [test] fn encode_static_header () { let mut encoded = [0 ; 3] ; Encoder :: default () . encode (& [(b":method" , b"GET")] , & mut encoded) . unwrap () ; assert_eq ! (encoded , [0 , 0 , INDEXED | 0x40 | 17]) ; } # [test] fn encode_static_header_name_only () { let mut encoded = [0 ; 11] ; let mut expected = [0 ; 11] ; let mut buf = octets :: OctetsMut :: with_slice (& mut expected [..]) ; buf . put_u16 (0) . unwrap () ; buf . put_u8 (LITERAL_WITH_NAME_REF | 0x10 | 15) . unwrap () ; buf . put_u8 (0) . unwrap () ; encode_str :: < false > (b"FORGET" , 0 , 7 , & mut buf) . unwrap () ; Encoder :: default () . encode (& [(b":method" , b"FORGET")] , & mut encoded) . unwrap () ; assert_eq ! (encoded , expected) ; } }
};
}
