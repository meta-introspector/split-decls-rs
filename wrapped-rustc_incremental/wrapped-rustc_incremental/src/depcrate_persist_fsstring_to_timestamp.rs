// Generated macro for string_to_timestamp (function)
macro_rules! Depcrate_persist_fsstring_to_timestamp {
() => {
// Module: crate::persist::fs
// Provides: {"string_to_timestamp"}
// Dependencies: {}
fn string_to_timestamp (s : & str) -> Result < SystemTime , & 'static str > { let micros_since_unix_epoch = match u64 :: from_str_radix (s , INT_ENCODE_BASE as u32) { Ok (micros) => micros , Err (_) => return Err ("timestamp not an int") , } ; let duration = Duration :: from_micros (micros_since_unix_epoch) ; Ok (UNIX_EPOCH + duration) }
};
}
