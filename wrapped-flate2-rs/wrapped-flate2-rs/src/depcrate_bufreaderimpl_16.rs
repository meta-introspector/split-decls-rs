// Generated macro for impl_16 (impl)
macro_rules! Depcrate_bufreaderimpl_16 {
() => {
// Module: crate::bufreader
// Provides: {"impl_16"}
// Dependencies: {}
impl < R : Read > Read for BufReader < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . pos == self . cap && buf . len () >= self . buf . len () { return self . inner . read (buf) ; } let nread = { let mut rem = self . fill_buf () ? ; rem . read (buf) ? } ; self . consume (nread) ; Ok (nread) } }
};
}
