// Generated macro for impl_26 (impl)
macro_rules! Depcrate_frameimpl_26 {
() => {
// Module: crate::frame
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < 'ctx , R > fallible_iterator :: FallibleIterator for FrameIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = Frame < 'ctx , R > ; type Error = Error ; # [inline] fn next (& mut self) -> Result < Option < Frame < 'ctx , R > > , Error > { self . next () } }
};
}
