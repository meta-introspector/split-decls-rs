// Generated macro for impl_319 (impl)
macro_rules! Depcrate_streamimpl_319 {
() => {
// Module: crate::stream
// Provides: {"impl_319"}
// Dependencies: {}
impl < S > ResetStream for MaybePartialStream < S > where S : ResetStream , { type Checkpoint = S :: Checkpoint ; # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . 0 . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , S :: Error > { self . 0 . reset (checkpoint) } }
};
}
