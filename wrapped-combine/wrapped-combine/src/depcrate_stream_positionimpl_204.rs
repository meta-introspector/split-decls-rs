// Generated macro for impl_204 (impl)
macro_rules! Depcrate_stream_positionimpl_204 {
() => {
// Module: crate::stream::position
// Provides: {"impl_204"}
// Dependencies: {}
impl < Item , T > Positioner < Item > for & '_ mut T where Item : Clone , T : ? Sized + Positioner < Item > , { type Position = T :: Position ; type Checkpoint = T :: Checkpoint ; # [inline] fn position (& self) -> T :: Position { (* * self) . position () } # [inline] fn update (& mut self , item : & Item) { (* * self) . update (item) } # [inline] fn checkpoint (& self) -> Self :: Checkpoint { (* * self) . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) { (* * self) . reset (checkpoint) } }
};
}
