// Generated macro for macro_439 (macro)
macro_rules! Depcrate_fut_try_future_map_errmacro_439 {
() => {
// Module: crate::fut::try_future::map_err
// Provides: {"macro_439"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`map`](super::ActorTryFutureExt::map_err) method."] # [project = MapProj] # [project_replace = MapProjReplace] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub enum MapErr < Fut , F > { Incomplete { # [pin] future : Fut , f : F , } , Complete , } }
};
}
