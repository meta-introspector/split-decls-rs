// Generated macro for impl_try_from_slice_unchecked (macro)
macro_rules! Depcrate_macrosimpl_try_from_slice_unchecked {
() => {
// Module: crate::macros
// Provides: {"impl_try_from_slice_unchecked"}
// Dependencies: {}
macro_rules ! impl_try_from_slice_unchecked { ($ borsh : ident , $ borsh_io : ident $ (,# [$ meta : meta]) ?) => { # [doc = " Deserializes without checking that the entire slice has been consumed"] # [doc = ""] # [doc = " Normally, `try_from_slice` checks the length of the final slice to ensure"] # [doc = " that the deserialization uses up all of the bytes in the slice."] # [doc = ""] # [doc = " Note that there is a potential issue with this function. Any buffer greater than"] # [doc = " or equal to the expected size will properly deserialize. For example, if the"] # [doc = " user passes a buffer destined for a different type, the error won't get caught"] # [doc = " as easily."] $ (# [$ meta]) ? pub fn try_from_slice_unchecked < T : $ borsh :: BorshDeserialize > (data : & [u8]) -> Result < T , $ borsh_io :: Error > { let mut data_mut = data ; let result = T :: deserialize (& mut data_mut) ?; Ok (result) } } }
};
}
