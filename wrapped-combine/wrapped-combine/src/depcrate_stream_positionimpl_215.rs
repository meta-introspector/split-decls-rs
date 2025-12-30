// Generated macro for impl_215 (impl)
macro_rules! Depcrate_stream_positionimpl_215 {
() => {
// Module: crate::stream::position
// Provides: {"impl_215"}
// Dependencies: {}
impl Positioner < u8 > for SourcePosition { type Position = SourcePosition ; type Checkpoint = Self ; # [inline] fn position (& self) -> SourcePosition { * self } # [inline] fn update (& mut self , token : & u8) { self . column += 1 ; if * token == b'\n' { self . column = 1 ; self . line += 1 ; } } # [inline] fn checkpoint (& self) -> Self :: Checkpoint { * self } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) { * self = checkpoint ; } }
};
}
