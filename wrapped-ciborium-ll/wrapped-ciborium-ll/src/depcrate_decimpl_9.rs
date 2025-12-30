// Generated macro for impl_9 (impl)
macro_rules! Depcrate_decimpl_9 {
() => {
// Module: crate::dec
// Provides: {"impl_9"}
// Dependencies: {}
impl < R : Read > Read for Decoder < R > { type Error = R :: Error ; # [inline] fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > { assert ! (self . buffer . is_none ()) ; self . reader . read_exact (data) ? ; self . offset += data . len () ; Ok (()) } }
};
}
