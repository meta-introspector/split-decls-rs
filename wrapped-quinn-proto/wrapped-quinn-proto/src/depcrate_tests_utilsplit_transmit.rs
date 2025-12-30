// Generated macro for split_transmit (function)
macro_rules! Depcrate_tests_utilsplit_transmit {
() => {
// Module: crate::tests::util
// Provides: {"split_transmit"}
// Dependencies: {}
fn split_transmit (transmit : Transmit , buffer : & [u8]) -> Vec < (Transmit , Bytes) > { let mut buffer = Bytes :: copy_from_slice (buffer) ; let segment_size = match transmit . segment_size { Some (segment_size) => segment_size , _ => return vec ! [(transmit , buffer)] , } ; let mut transmits = Vec :: new () ; while ! buffer . is_empty () { let end = segment_size . min (buffer . len ()) ; let contents = buffer . split_to (end) ; transmits . push ((Transmit { destination : transmit . destination , size : contents . len () , ecn : transmit . ecn , segment_size : None , src_ip : transmit . src_ip , } , contents ,)) ; } transmits }
};
}
