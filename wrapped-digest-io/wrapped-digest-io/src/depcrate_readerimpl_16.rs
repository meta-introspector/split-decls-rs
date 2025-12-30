// Generated macro for impl_16 (impl)
macro_rules! Depcrate_readerimpl_16 {
() => {
// Module: crate::reader
// Provides: {"impl_16"}
// Dependencies: {}
impl < D : Digest , R : io :: BufRead > HashReader < D , R > { # [doc = " Read and hash all bytes remaining in the reader, discarding the data"] # [doc = " Based on implementation in b2sum crate, MIT License Copyright (c) 2017 John Downey"] pub fn hash_to_end (& mut self) { loop { let count = { let data = self . reader . fill_buf () . unwrap () ; if data . is_empty () { break ; } self . hasher . update (data) ; data . len () } ; self . reader . consume (count) ; } } }
};
}
