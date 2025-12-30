// Generated macro for BodyStates (struct)
macro_rules! Depcrate_nbodyBodyStates {
() => {
// Module: crate::nbody
// Provides: {"BodyStates"}
// Dependencies: {}
# [doc = " The main structure of this program."] # [doc = " The state for a body is split across"] # [doc = " a position vector,"] # [doc = " a velocities vector,"] # [doc = " and a mass vector."] # [doc = ""] # [doc = " Originally, this was implemented as an array of structures—now"] # [doc = " it's a structure of arrays."] # [doc = " This was both for testing optimizations and for minute practice with EC(S)."] pub struct BodyStates { poss : Vec < Position > , vels : Vec < Velocity > , masses : Vec < Number > , }
};
}
