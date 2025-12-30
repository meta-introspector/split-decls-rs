// Generated macro for macro_1508 (macro)
macro_rules! Depcrate_stream_try_stream_try_anymacro_1508 {
() => {
// Module: crate::stream::try_stream::try_any
// Provides: {"macro_1508"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_any`](super::TryStreamExt::try_any) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryAny < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
};
}
