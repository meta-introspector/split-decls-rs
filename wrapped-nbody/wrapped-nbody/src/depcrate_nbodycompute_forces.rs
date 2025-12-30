// Generated macro for compute_forces (function)
macro_rules! Depcrate_nbodycompute_forces {
() => {
// Module: crate::nbody
// Provides: {"compute_forces"}
// Dependencies: {}
# [doc = " Computes the next `BodyStates`."] pub fn compute_forces (bs : BodyStates) -> BodyStates { let accs = accelerations (& bs) ; BodyStates { poss : bs . poss . iter () . zip (bs . vels . iter ()) . map (| (p , v) | move_position (p , v)) . collect () , vels : bs . vels . iter () . zip (accs . iter ()) . map (| (v , a) | update_velocity (v , a)) . collect () , masses : bs . masses , } }
};
}
