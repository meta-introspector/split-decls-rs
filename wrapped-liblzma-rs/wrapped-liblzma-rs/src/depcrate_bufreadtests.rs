// Generated macro for tests (module)
macro_rules! Depcrate_bufreadtests {
() => {
// Module: crate::bufread
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [cfg (all (target_family = "wasm" , target_os = "unknown"))] use wasm_bindgen_test :: wasm_bindgen_test as test ; # [test] fn compressed_and_trailing_data () { let mut to_compress : Vec < u8 > = Vec :: new () ; const COMPRESSED_ORIG_SIZE : usize = 1024 ; for num in 0 .. COMPRESSED_ORIG_SIZE { to_compress . push (num as u8) } let mut encoder = XzEncoder :: new (& to_compress [..] , 6) ; let mut decoder_input = Vec :: new () ; encoder . read_to_end (& mut decoder_input) . unwrap () ; assert_eq ! (encoder . total_in () , to_compress . len () as u64) ; assert_eq ! (encoder . total_out () , decoder_input . len () as u64) ; const ADDITIONAL_SIZE : usize = 123 ; let mut additional_data = Vec :: new () ; for num in 0 .. ADDITIONAL_SIZE { additional_data . push (((25 + num) % 256) as u8) } decoder_input . extend (& additional_data) ; let mut decoder_reader = & decoder_input [..] ; { let mut decoder = XzDecoder :: new (& mut decoder_reader) ; let mut decompressed_data = vec ! [0u8 ; to_compress . len ()] ; assert_eq ! (decoder . read (& mut decompressed_data) . unwrap () , COMPRESSED_ORIG_SIZE) ; assert_eq ! (decompressed_data , & to_compress [..]) ; assert_eq ! (decoder . total_in () , (decoder_input . len () - ADDITIONAL_SIZE) as u64) ; assert_eq ! (decoder . total_out () , decompressed_data . len () as u64) ; } let mut remaining_data = Vec :: new () ; let nb_read = decoder_reader . read_to_end (& mut remaining_data) . unwrap () ; assert_eq ! (nb_read , ADDITIONAL_SIZE) ; assert_eq ! (remaining_data , & additional_data [..]) ; } }
};
}
