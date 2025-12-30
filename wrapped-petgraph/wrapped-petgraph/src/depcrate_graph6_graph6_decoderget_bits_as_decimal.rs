// Generated macro for get_bits_as_decimal (function)
macro_rules! Depcrate_graph6_graph6_decoderget_bits_as_decimal {
() => {
// Module: crate::graph6::graph6_decoder
// Provides: {"get_bits_as_decimal"}
// Dependencies: {}
fn get_bits_as_decimal (bits : Vec < u8 >) -> usize { let bits_str = bits . iter () . map (| bit | bit . to_string ()) . collect :: < Vec < String > > () . join ("") ; usize :: from_str_radix (& bits_str , 2) . unwrap () }
};
}
