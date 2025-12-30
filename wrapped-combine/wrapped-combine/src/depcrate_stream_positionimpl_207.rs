// Generated macro for impl_207 (impl)
macro_rules! Depcrate_stream_positionimpl_207 {
() => {
// Module: crate::stream::position
// Provides: {"impl_207"}
// Dependencies: {}
impl < Item > Positioner < Item > for IndexPositioner where Item : Clone , { type Position = usize ; type Checkpoint = Self ; # [inline] fn position (& self) -> usize { self . 0 } # [inline] fn update (& mut self , _item : & Item) { self . 0 += 1 } # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . clone () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) { * self = checkpoint ; } }
};
}
