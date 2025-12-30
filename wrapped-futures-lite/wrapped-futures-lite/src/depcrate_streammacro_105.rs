// Generated macro for macro_105 (macro)
macro_rules! Depcrate_streammacro_105 {
() => {
// Module: crate::stream
// Provides: {"macro_105"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`once_future()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct OnceFuture < F > { # [pin] future : Option < F >, } }
};
}
