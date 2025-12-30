// Generated macro for open_within (function)
macro_rules! Depcrate_aeadopen_within {
() => {
// Module: crate::aead
// Provides: {"open_within"}
// Dependencies: {}
fn open_within < 'o > (in_out : Overlapping < 'o > , Tag (received_tag) : & Tag , forged_plaintext : ForgedPlaintext , open : impl FnOnce (Overlapping) -> Result < Tag , InputTooLongError > ,) -> Result < & 'o mut [u8] , AuthError > { let in_out_len = in_out . len () ; let (plaintext , calculated_tag) = in_out . assume_entire_output_written_on_success (open) . map_err (| InputTooLongError { .. } | AuthError :: new (in_out_len)) ? ; match bb :: verify_slices_are_equal (calculated_tag . as_ref () , received_tag . as_ref ()) { Ok (()) => Ok (plaintext) , Err (_) => { match forged_plaintext { ForgedPlaintext :: Zero => plaintext . fill (0) , } Err (AuthError :: new (in_out_len)) } } }
};
}
