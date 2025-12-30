// Generated macro for impl_290 (impl)
macro_rules! Depcrate_streamimpl_290 {
() => {
// Module: crate::stream
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'a , I > ResetStream for & 'a mut I where I : ResetStream + ? Sized , { type Checkpoint = I :: Checkpoint ; fn checkpoint (& self) -> Self :: Checkpoint { (* * self) . checkpoint () } fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { (* * self) . reset (checkpoint) } }
};
}
