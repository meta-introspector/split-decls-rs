macro_rules! deps {
    () => {
        UnableToWriteDebuggerVisualizer!();
    };
}

macro_rules! collect_natvis_visualizers {
    () => {
        deps!();
        fn collect_natvis_visualizers (tmpdir : & Path , sess : & Session , crate_name : & Symbol , natvis_debugger_visualizers : & BTreeSet < DebuggerVisualizerFile > ,) -> Vec < PathBuf > { let mut visualizer_paths = Vec :: with_capacity (natvis_debugger_visualizers . len ()) ; for (index , visualizer) in natvis_debugger_visualizers . iter () . enumerate () { let visualizer_out_file = tmpdir . join (format ! ("{}-{}.natvis" , crate_name . as_str () , index)) ; match fs :: write (& visualizer_out_file , & visualizer . src) { Ok (()) => { visualizer_paths . push (visualizer_out_file) ; } Err (error) => { sess . dcx () . emit_warn (errors :: UnableToWriteDebuggerVisualizer { path : visualizer_out_file , error , }) ; } } ; } visualizer_paths }
    };
}

collect_natvis_visualizers!();