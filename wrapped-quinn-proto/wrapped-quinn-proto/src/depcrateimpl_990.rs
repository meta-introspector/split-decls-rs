// Generated macro for impl_990 (impl)
macro_rules! Depcrateimpl_990 {
() => {
// Module: crate
// Provides: {"impl_990"}
// Dependencies: {}
impl StreamId { # [doc = " Create a new StreamId"] pub fn new (initiator : Side , dir : Dir , index : u64) -> Self { Self ((index << 2) | ((dir as u64) << 1) | initiator as u64) } # [doc = " Which side of a connection initiated the stream"] pub fn initiator (self) -> Side { if self . 0 & 0x1 == 0 { Side :: Client } else { Side :: Server } } # [doc = " Which directions data flows in"] pub fn dir (self) -> Dir { if self . 0 & 0x2 == 0 { Dir :: Bi } else { Dir :: Uni } } # [doc = " Distinguishes streams of the same initiator and directionality"] pub fn index (self) -> u64 { self . 0 >> 2 } }
};
}
