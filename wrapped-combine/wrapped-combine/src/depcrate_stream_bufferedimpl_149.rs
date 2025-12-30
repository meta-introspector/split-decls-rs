// Generated macro for impl_149 (impl)
macro_rules! Depcrate_stream_bufferedimpl_149 {
() => {
// Module: crate::stream::buffered
// Provides: {"impl_149"}
// Dependencies: {}
impl < Input > ResetStream for Stream < Input > where Input : Positioned , { type Checkpoint = usize ; fn checkpoint (& self) -> Self :: Checkpoint { self . offset } fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { if checkpoint < self . buffer_offset - self . buffer . len () { Err (Self :: Error :: from_error (self . position () , StreamErrorFor :: < Self > :: message_static_message ("Backtracked to far") ,)) } else { self . offset = checkpoint ; Ok (()) } } }
};
}
