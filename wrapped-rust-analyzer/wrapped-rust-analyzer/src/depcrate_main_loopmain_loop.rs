// Generated macro for main_loop (function)
macro_rules! Depcrate_main_loopmain_loop {
() => {
// Module: crate::main_loop
// Provides: {"main_loop"}
// Dependencies: {}
pub fn main_loop (config : Config , connection : Connection) -> anyhow :: Result < () > { tracing :: info ! ("initial config: {:#?}" , config) ; # [cfg (windows)] unsafe { use windows_sys :: Win32 :: System :: Threading :: * ; let thread = GetCurrentThread () ; let thread_priority_above_normal = 1 ; SetThreadPriority (thread , thread_priority_above_normal) ; } # [cfg (feature = "dhat")] { if let Some (dhat_output_file) = config . dhat_output_file () { * crate :: DHAT_PROFILER . lock () . unwrap () = Some (dhat :: Profiler :: builder () . file_name (& dhat_output_file) . build ()) ; } } GlobalState :: new (connection . sender , config) . run (connection . receiver) }
};
}
