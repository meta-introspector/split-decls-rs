// Generated macro for accelerations (function)
macro_rules! Depcrate_nbodyaccelerations {
() => {
// Module: crate::nbody
// Provides: {"accelerations"}
// Dependencies: {}
# [doc = " Calculate the accelerations for these bodies—used to update the bodies' velocities."] fn accelerations (bs : & BodyStates) -> Vec < Acceleration > { bs . poss . iter () . zip (bs . masses . iter ()) . map (| (ref p , & m) | { let reference = bs . poss . iter () . zip (bs . masses . iter ()) ; let Force { fx , fy , fz } = forces_for_body (p , m , reference) ; Acceleration { ax : fx / m , ay : fy / m , az : fz / m , } }) . collect () }
};
}
