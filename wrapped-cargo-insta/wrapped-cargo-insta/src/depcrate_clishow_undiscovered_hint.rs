// Generated macro for show_undiscovered_hint (function)
macro_rules! Depcrate_clishow_undiscovered_hint {
() => {
// Module: crate::cli
// Provides: {"show_undiscovered_hint"}
// Dependencies: {}
fn show_undiscovered_hint (find_flags : FindFlags , snapshot_containers : & [SnapshotContainer] , roots : & HashSet < PathBuf > , extensions : & [& str] ,) { if find_flags . include_hidden && find_flags . include_ignored { return ; } let found_snapshots = snapshot_containers . iter () . filter_map (| x | x . snapshot_file ()) . map (| x | x . to_path_buf ()) . collect :: < HashSet < _ > > () ; let all_snapshots : HashSet < _ > = roots . iter () . flat_map (| root | { make_snapshot_walker (root , extensions , FindFlags { include_ignored : true , include_hidden : true , } ,) . filter_map (| e | e . ok ()) . filter (| x | { let fname = x . file_name () . to_string_lossy () ; extensions . iter () . any (| ext | fname . ends_with (& format ! (".{ext}.new"))) || fname . ends_with (".pending-snap") }) . map (| x | x . path () . to_path_buf ()) }) . collect () ; let missed_snapshots = all_snapshots . difference (& found_snapshots) . collect_vec () ; if missed_snapshots . is_empty () { return ; } let (args , paths) = match (find_flags . include_ignored , find_flags . include_hidden) { (false , true) => ("--include-ignored" , "ignored") , (true , false) => ("--include-hidden" , "hidden") , (false , false) => ("--include-ignored and --include-hidden" , "ignored or hidden" ,) , (true , true) => unreachable ! () , } ; eprintln ! ("{}: {}" , style ("warning") . yellow () . bold () , format_args ! ("found undiscovered pending snapshots in some paths which are not picked up by cargo \
            insta. Use {} if you have snapshots in {} paths. Files:\n{}" , args , paths , missed_snapshots . iter () . map (| x | x . display () . to_string ()) . join ("\n"))) ; }
};
}
