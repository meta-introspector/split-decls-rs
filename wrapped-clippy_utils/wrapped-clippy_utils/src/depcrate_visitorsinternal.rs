// Generated macro for internal (module)
macro_rules! Depcrate_visitorsinternal {
() => {
// Module: crate::visitors
// Provides: {"internal"}
// Dependencies: {}
mod internal { # [doc = " Trait for visitor functions to control whether or not to descend to child nodes. Implemented"] # [doc = " for only two types. `()` always descends. `Descend` allows controlled descent."] pub trait Continue { fn descend (& self) -> bool ; } }
};
}
