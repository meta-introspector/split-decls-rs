// Generated macro for impl_16 (impl)
macro_rules! Depcrate_entryimpl_16 {
() => {
// Module: crate::entry
// Provides: {"impl_16"}
// Dependencies: {}
impl std :: io :: Read for Entry < '_ > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let buf_len = buf . len () ; if let Some (err) = self . parent . err . lock () . take () { return Err (std :: io :: Error :: other (err)) ; } let bytes_read = match self . remaining . as_mut () { None => { let input = self . fill_buf () ? ; let nb = input . len () . min (buf . len ()) ; buf [.. nb] . copy_from_slice (& input [.. nb]) ; self . parent . pos += nb ; nb } Some (remaining) => { let bytes_read = self . parent . read . read (& mut buf [.. buf_len . min (* remaining)]) ? ; * remaining -= bytes_read ; bytes_read } } ; if bytes_read == 0 { self . remaining = Some (0) ; } Ok (bytes_read) } }
};
}
