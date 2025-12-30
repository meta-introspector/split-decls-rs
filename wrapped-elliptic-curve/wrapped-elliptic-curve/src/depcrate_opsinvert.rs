// Generated macro for invert (function)
macro_rules! Depcrate_opsinvert {
() => {
// Module: crate::ops
// Provides: {"invert"}
// Dependencies: {}
fn invert < T : Field > (scalar : T) -> (T , Choice) { let scalar = scalar . invert () ; let choice = scalar . is_some () ; let scalar = scalar . unwrap_or (T :: default ()) ; (scalar , choice) }
};
}
