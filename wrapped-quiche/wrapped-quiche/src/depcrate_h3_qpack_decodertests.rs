// Generated macro for tests (module)
macro_rules! Depcrate_h3_qpack_decodertests {
() => {
// Module: crate::h3::qpack::decoder
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn decode_int1 () { let encoded = [0b01010 , 0x02] ; let mut b = octets :: Octets :: with_slice (& encoded) ; assert_eq ! (decode_int (& mut b , 5) , Ok (10)) ; } # [test] fn decode_int2 () { let encoded = [0b11111 , 0b10011010 , 0b00001010] ; let mut b = octets :: Octets :: with_slice (& encoded) ; assert_eq ! (decode_int (& mut b , 5) , Ok (1337)) ; } # [test] fn decode_int3 () { let encoded = [0b101010] ; let mut b = octets :: Octets :: with_slice (& encoded) ; assert_eq ! (decode_int (& mut b , 8) , Ok (42)) ; } }
};
}
