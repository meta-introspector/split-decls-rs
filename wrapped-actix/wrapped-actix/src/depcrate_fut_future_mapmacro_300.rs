// Generated macro for macro_300 (macro)
macro_rules! Depcrate_fut_future_mapmacro_300 {
() => {
// Module: crate::fut::future::map
// Provides: {"macro_300"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`map`](super::ActorFutureExt::map) method."] # [project = MapProj] # [project_replace = MapProjReplace] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub enum Map < Fut , F > { Incomplete { # [pin] future : Fut , f : F , } , Complete , } }
};
}
