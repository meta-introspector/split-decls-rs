// Generated macro for impl_287 (impl)
macro_rules! Depcrate_read_cfiimpl_287 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_287"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < 'a , R : Reader > fallible_iterator :: FallibleIterator for CallFrameInstructionIter < 'a , R > { type Item = CallFrameInstruction < R :: Offset > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { CallFrameInstructionIter :: next (self) } }
};
}
