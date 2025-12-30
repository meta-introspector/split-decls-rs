// Generated macro for ResetStream (trait)
macro_rules! Depcrate_streamResetStream {
() => {
// Module: crate::stream
// Provides: {"ResetStream"}
// Dependencies: {}
# [doc = " A `StreamOnce` which can create checkpoints which the stream can be reset to"] pub trait ResetStream : StreamOnce { type Checkpoint : Clone ; # [doc = " Creates a `Checkpoint` at the current position which can be used to reset the stream"] # [doc = " later to the current position"] fn checkpoint (& self) -> Self :: Checkpoint ; # [doc = " Attempts to reset the stream to an earlier position."] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > ; }
};
}
