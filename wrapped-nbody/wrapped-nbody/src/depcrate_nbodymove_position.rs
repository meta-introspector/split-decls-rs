// Generated macro for move_position (function)
macro_rules! Depcrate_nbodymove_position {
() => {
// Module: crate::nbody
// Provides: {"move_position"}
// Dependencies: {}
# [doc = " Returns a new `Position` from a `Position` moving at a certain `Velocity`."] fn move_position (p : & Position , v : & Velocity) -> Position { Position { x : p . x + v . dx * TIMESTEP , y : p . y + v . dy * TIMESTEP , z : p . z + v . dz * TIMESTEP , } }
};
}
