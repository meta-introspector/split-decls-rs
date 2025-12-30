// Generated macro for impl_247 (impl)
macro_rules! Depcrate_stream_stateimpl_247 {
() => {
// Module: crate::stream::state
// Provides: {"impl_247"}
// Dependencies: {}
impl < S , U > ResetStream for Stream < S , U > where S : ResetStream , { type Checkpoint = S :: Checkpoint ; # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . stream . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { self . stream . reset (checkpoint) } }
};
}
