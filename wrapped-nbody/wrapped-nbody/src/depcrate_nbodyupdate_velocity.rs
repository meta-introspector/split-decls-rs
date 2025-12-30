// Generated macro for update_velocity (function)
macro_rules! Depcrate_nbodyupdate_velocity {
() => {
// Module: crate::nbody
// Provides: {"update_velocity"}
// Dependencies: {}
# [doc = " Returns a new `Velocity` from a `Velocity` accelerating at a certain `Acceleration`."] fn update_velocity (v : & Velocity , a : & Acceleration) -> Velocity { Velocity { dx : v . dx + a . ax * TIMESTEP , dy : v . dy + a . ay * TIMESTEP , dz : v . dz + a . az * TIMESTEP , } }
};
}
