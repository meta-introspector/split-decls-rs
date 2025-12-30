// Generated macro for read_and_pass_to (function)
macro_rules! Depcrate_data_input_bytes_to_entriesread_and_pass_to {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"read_and_pass_to"}
// Dependencies: {}
fn read_and_pass_to < R : io :: Read , W : io :: Write > (read : & mut R , to : W) -> PassThrough < & mut R , W > { PassThrough { read , write : to } }
};
}
