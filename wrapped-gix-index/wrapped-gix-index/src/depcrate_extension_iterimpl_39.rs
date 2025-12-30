// Generated macro for impl_39 (impl)
macro_rules! Depcrate_extension_iterimpl_39 {
() => {
// Module: crate::extension::iter
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > Iter < 'a > { # [doc = " Create a new extension iterator at the entrypoint for extensions until the end of the extensions."] pub fn new (data_at_beginning_of_extensions_and_truncated : & 'a [u8]) -> Self { Iter { data : data_at_beginning_of_extensions_and_truncated , consumed : 0 , } } # [doc = " Create a new iterator at with a data block to the end of the file, and we automatically remove the trailing"] # [doc = " hash of type `object_hash`."] pub fn new_without_checksum (data_at_beginning_of_extensions : & 'a [u8] , object_hash : gix_hash :: Kind ,) -> Option < Self > { let end = data_at_beginning_of_extensions . len () . checked_sub (object_hash . len_in_bytes ()) ? ; Iter { data : & data_at_beginning_of_extensions [.. end] , consumed : 0 , } . into () } }
};
}
