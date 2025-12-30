// Generated macro for impl_307 (impl)
macro_rules! Depcrate_streamimpl_307 {
() => {
// Module: crate::stream
// Provides: {"impl_307"}
// Dependencies: {}
impl < S > ResetStream for PartialStream < S > where S : ResetStream , { type Checkpoint = S :: Checkpoint ; # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . 0 . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , S :: Error > { self . 0 . reset (checkpoint) } }
};
}
