// Generated macro for impl_214 (impl)
macro_rules! Depcrate_stream_positionimpl_214 {
() => {
// Module: crate::stream::position
// Provides: {"impl_214"}
// Dependencies: {}
impl Positioner < char > for SourcePosition { type Position = SourcePosition ; type Checkpoint = Self ; # [inline] fn position (& self) -> SourcePosition { * self } # [inline] fn update (& mut self , token : & char) { self . column += 1 ; if * token == '\n' { self . column = 1 ; self . line += 1 ; } } # [inline] fn checkpoint (& self) -> Self :: Checkpoint { * self } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) { * self = checkpoint ; } }
};
}
