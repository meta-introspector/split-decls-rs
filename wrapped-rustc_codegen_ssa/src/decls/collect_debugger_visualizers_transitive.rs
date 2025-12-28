macro_rules! collect_debugger_visualizers_transitive {
    () => {
        # [doc = " This function returns all of the debugger visualizers specified for the"] # [doc = " current crate as well as all upstream crates transitively that match the"] # [doc = " `visualizer_type` specified."] pub fn collect_debugger_visualizers_transitive (tcx : TyCtxt < '_ > , visualizer_type : DebuggerVisualizerType ,) -> BTreeSet < DebuggerVisualizerFile > { tcx . debugger_visualizers (LOCAL_CRATE) . iter () . chain (tcx . crates (()) . iter () . filter (| & cnum | { let used_crate_source = tcx . used_crate_source (* cnum) ; used_crate_source . rlib . is_some () || used_crate_source . rmeta . is_some () }) . flat_map (| & cnum | tcx . debugger_visualizers (cnum)) ,) . filter (| visualizer | visualizer . visualizer_type == visualizer_type) . cloned () . collect :: < BTreeSet < _ > > () }
    };
}

collect_debugger_visualizers_transitive!();