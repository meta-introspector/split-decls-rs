// Generated macro for encode_to_iter (function)
macro_rules! Depcrateencode_to_iter {
() => {
// Module: crate
// Provides: {"encode_to_iter"}
// Dependencies: {}
fn encode_to_iter < T : iter :: FromIterator < char > > (table : & 'static [u8 ; 16] , source : & [u8]) -> T { BytesToHexChars :: new (source , table) . collect () }
};
}
