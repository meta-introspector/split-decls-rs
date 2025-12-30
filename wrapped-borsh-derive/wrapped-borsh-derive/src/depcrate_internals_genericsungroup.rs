// Generated macro for ungroup (function)
macro_rules! Depcrate_internals_genericsungroup {
() => {
// Module: crate::internals::generics
// Provides: {"ungroup"}
// Dependencies: {}
fn ungroup (mut ty : & Type) -> & Type { while let Type :: Group (group) = ty { ty = & group . elem ; } ty }
};
}
