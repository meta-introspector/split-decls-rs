// Generated macro for macro_447 (macro)
macro_rules! Depcrate_fut_try_future_map_okmacro_447 {
() => {
// Module: crate::fut::try_future::map_ok
// Provides: {"macro_447"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`map`](super::ActorTryFutureExt::map_ok) method."] # [project = MapProj] # [project_replace = MapProjReplace] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub enum MapOk < Fut , F > { Incomplete { # [pin] future : Fut , f : F , } , Complete , } }
};
}
