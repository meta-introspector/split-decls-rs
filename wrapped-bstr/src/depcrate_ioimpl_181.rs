// Generated macro for impl_181 (impl)
macro_rules! Depcrate_ioimpl_181 {
() => {
// Module: crate::io
// Provides: {"impl_181"}
// Dependencies: {}
impl < B : io :: BufRead > Iterator for ByteRecords < B > { type Item = io :: Result < Vec < u8 > > ; fn next (& mut self) -> Option < io :: Result < Vec < u8 > > > { let mut bytes = vec ! [] ; match self . buf . read_until (self . terminator , & mut bytes) { Err (e) => Some (Err (e)) , Ok (0) => None , Ok (_) => { trim_record (& mut bytes , self . terminator) ; Some (Ok (bytes)) } } } }
};
}
