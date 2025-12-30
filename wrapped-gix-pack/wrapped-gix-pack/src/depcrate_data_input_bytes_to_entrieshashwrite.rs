// Generated macro for HashWrite (struct)
macro_rules! Depcrate_data_input_bytes_to_entriesHashWrite {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"HashWrite"}
// Dependencies: {}
# [doc = " A utility to automatically generate a hash while writing into an inner writer."] pub struct HashWrite < 'a , T > { # [doc = " The hash implementation."] pub hash : & 'a mut Hasher , # [doc = " The inner writer."] pub inner : T , }
};
}
