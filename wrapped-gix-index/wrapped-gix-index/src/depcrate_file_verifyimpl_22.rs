// Generated macro for impl_22 (impl)
macro_rules! Depcrate_file_verifyimpl_22 {
() => {
// Module: crate::file::verify
// Provides: {"impl_22"}
// Dependencies: {}
impl File { # [doc = " Verify the integrity of the index to assure its consistency."] pub fn verify_integrity (& self) -> Result < () , Error > { let _span = gix_features :: trace :: coarse ! ("gix_index::File::verify_integrity()") ; if let Some (checksum) = self . checksum { let num_bytes_to_hash = self . path . metadata () . map_err (gix_hash :: io :: Error :: from) ? . len () - checksum . as_bytes () . len () as u64 ; let should_interrupt = AtomicBool :: new (false) ; gix_hash :: bytes_of_file (& self . path , num_bytes_to_hash , checksum . kind () , & mut gix_features :: progress :: Discard , & should_interrupt ,) ? . verify (& checksum) ? ; } Ok (()) } }
};
}
