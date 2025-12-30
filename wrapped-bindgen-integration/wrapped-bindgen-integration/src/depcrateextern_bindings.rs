// Generated macro for extern_bindings (module)
macro_rules! Depcrateextern_bindings {
() => {
// Module: crate
// Provides: {"extern_bindings"}
// Dependencies: {}
mod extern_bindings { include ! (concat ! (env ! ("OUT_DIR") , "/extern.rs")) ; }
};
}
