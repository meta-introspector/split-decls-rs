// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl Read for & [u8] { type Error = EndOfFile ; # [inline] fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > { if data . len () > self . len () { return Err (EndOfFile (())) ; } let (prefix , suffix) = self . split_at (data . len ()) ; data . copy_from_slice (prefix) ; * self = suffix ; Ok (()) } }
};
}
