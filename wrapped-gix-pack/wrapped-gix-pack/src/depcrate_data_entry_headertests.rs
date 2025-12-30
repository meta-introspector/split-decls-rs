// Generated macro for tests (module)
macro_rules! Depcrate_data_entry_headertests {
() => {
// Module: crate::data::entry::header
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn leb64_encode_max_int () { let mut buf = [0u8 ; 10] ; let buf = leb64_encode (u64 :: MAX , & mut buf) ; assert_eq ! (buf . len () , 10 , "10 bytes should be used when 64bits are encoded") ; } }
};
}
