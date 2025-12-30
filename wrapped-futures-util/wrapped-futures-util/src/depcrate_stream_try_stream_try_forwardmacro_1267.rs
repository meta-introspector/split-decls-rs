// Generated macro for macro_1267 (macro)
macro_rules! Depcrate_stream_try_stream_try_forwardmacro_1267 {
() => {
// Module: crate::stream::try_stream::try_forward
// Provides: {"macro_1267"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_forward`](super::TryStreamExt::try_forward) method."] # [project = TryForwardProj] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForward < St , Si , Item > { # [pin] sink : Option < Si >, # [pin] stream : Fuse < IntoStream < St >>, buffered_item : Option < Item >, } }
};
}
