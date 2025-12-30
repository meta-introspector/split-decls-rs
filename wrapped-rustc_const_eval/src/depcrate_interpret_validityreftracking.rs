// Generated macro for RefTracking (struct)
macro_rules! Depcrate_interpret_validityRefTracking {
() => {
// Module: crate::interpret::validity
// Provides: {"RefTracking"}
// Dependencies: {}
# [doc = " State for tracking recursive validation of references"] pub struct RefTracking < T , PATH = () > { seen : FxHashSet < T > , todo : Vec < (T , PATH) > , }
};
}
