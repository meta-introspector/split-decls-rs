// Generated macro for impl_314 (impl)
macro_rules! Depcrate_streamimpl_314 {
() => {
// Module: crate::stream
// Provides: {"impl_314"}
// Dependencies: {}
impl < S > ResetStream for CompleteStream < S > where S : ResetStream , { type Checkpoint = S :: Checkpoint ; # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . 0 . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , S :: Error > { self . 0 . reset (checkpoint) } }
};
}
