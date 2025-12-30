// Generated macro for macro_208 (macro)
macro_rules! Depcrate_streammacro_208 {
() => {
// Module: crate::stream
// Provides: {"macro_208"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::unzip()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct UnzipFuture < S , FromA , FromB > { # [pin] stream : S , res : Option < (FromA , FromB) >, } }
};
}
