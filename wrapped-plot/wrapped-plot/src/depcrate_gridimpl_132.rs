// Generated macro for impl_132 (impl)
macro_rules! Depcrate_gridimpl_132 {
() => {
// Module: crate::grid
// Provides: {"impl_132"}
// Dependencies: {}
impl Script for (Axis , Grid , & Properties) { fn script (& self) -> String { let & (axis , grid , properties) = self ; let axis = axis . display () ; let grid = grid . display () ; if properties . hidden { String :: new () } else { format ! ("set grid {}{}tics\n" , grid , axis) } } }
};
}
