// Generated macro for collect_natvis_visualizers (function)
macro_rules! Depcrate_back_linkcollect_natvis_visualizers {
() => {
// Module: crate::back::link
// Provides: {"collect_natvis_visualizers"}
// Dependencies: {}
fn collect_natvis_visualizers (tmpdir : & Path , sess : & Session , crate_name : & Symbol , natvis_debugger_visualizers : & BTreeSet < DebuggerVisualizerFile > ,) -> Vec < PathBuf > { let mut visualizer_paths = Vec :: with_capacity (natvis_debugger_visualizers . len ()) ; for (index , visualizer) in natvis_debugger_visualizers . iter () . enumerate () { let visualizer_out_file = tmpdir . join (format ! ("{}-{}.natvis" , crate_name . as_str () , index)) ; match fs :: write (& visualizer_out_file , & visualizer . src) { Ok (()) => { visualizer_paths . push (visualizer_out_file) ; } Err (error) => { sess . dcx () . emit_warn (errors :: UnableToWriteDebuggerVisualizer { path : visualizer_out_file , error , }) ; } } ; } visualizer_paths }
};
}
