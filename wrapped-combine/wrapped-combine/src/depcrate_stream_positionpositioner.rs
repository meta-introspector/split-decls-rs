// Generated macro for Positioner (trait)
macro_rules! Depcrate_stream_positionPositioner {
() => {
// Module: crate::stream::position
// Provides: {"Positioner"}
// Dependencies: {}
# [doc = " Trait for tracking the current position of a `Stream`."] pub trait Positioner < Item > { # [doc = " The type which keeps track of the position"] type Position : Clone + Ord ; type Checkpoint : Clone ; # [doc = " Returns the current position"] fn position (& self) -> Self :: Position ; # [doc = " Updates the position given that `token` has been taken from the stream"] fn update (& mut self , token : & Item) ; fn checkpoint (& self) -> Self :: Checkpoint ; fn reset (& mut self , checkpoint : Self :: Checkpoint) ; }
};
}
