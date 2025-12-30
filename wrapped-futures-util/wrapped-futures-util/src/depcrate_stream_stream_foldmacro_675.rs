// Generated macro for macro_675 (macro)
macro_rules! Depcrate_stream_stream_foldmacro_675 {
() => {
// Module: crate::stream::stream::fold
// Provides: {"macro_675"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`fold`](super::StreamExt::fold) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Fold < St , Fut , T , F > { # [pin] stream : St , f : F , accum : Option < T >, # [pin] future : Option < Fut >, } }
};
}
