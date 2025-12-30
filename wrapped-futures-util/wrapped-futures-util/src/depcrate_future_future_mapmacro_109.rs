// Generated macro for macro_109 (macro)
macro_rules! Depcrate_future_future_mapmacro_109 {
() => {
// Module: crate::future::future::map
// Provides: {"macro_109"}
// Dependencies: {}
pin_project ! { # [doc = " Internal Map future"] # [project = MapProj] # [project_replace = MapProjReplace] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub enum Map < Fut , F > { Incomplete { # [pin] future : Fut , f : F , } , Complete , } }
};
}
