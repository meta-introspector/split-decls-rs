// Generated macro for impl_184 (impl)
macro_rules! Depcrate_stream_easyimpl_184 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_184"}
// Dependencies: {}
impl < S > ResetStream for Stream < S > where S : ResetStream + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , { type Checkpoint = S :: Checkpoint ; fn checkpoint (& self) -> Self :: Checkpoint { self . 0 . checkpoint () } fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { self . 0 . reset (checkpoint) . map_err (crate :: error :: ParseError :: into_other) } }
};
}
