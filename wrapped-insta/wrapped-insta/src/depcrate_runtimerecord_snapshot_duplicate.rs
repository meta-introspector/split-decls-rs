// Generated macro for record_snapshot_duplicate (function)
macro_rules! Depcrate_runtimerecord_snapshot_duplicate {
() => {
// Module: crate::runtime
// Provides: {"record_snapshot_duplicate"}
// Dependencies: {}
fn record_snapshot_duplicate (results : & mut BTreeMap < String , Snapshot > , snapshot : & Snapshot , ctx : & SnapshotAssertionContext ,) { let key = ctx . duplication_key . as_deref () . unwrap () ; if let Some (prev_snapshot) = results . get (key) { if prev_snapshot . contents () != snapshot . contents () { println ! ("Snapshots in allow-duplicates block do not match.") ; let mut printer = SnapshotPrinter :: new (ctx . workspace , Some (prev_snapshot) , snapshot) ; printer . set_line (Some (ctx . assertion_line)) ; printer . set_snapshot_file (ctx . snapshot_file . as_deref ()) ; printer . set_title (Some ("Differences in Block")) ; printer . set_snapshot_hints ("previous assertion" , "current assertion") ; if ctx . tool_config . output_behavior () == OutputBehavior :: Diff { printer . set_show_diff (true) ; } printer . print () ; panic ! ("snapshot assertion for '{}' failed in line {}. Result \
                    does not match previous snapshot in allow-duplicates block." , ctx . snapshot_name . as_deref () . unwrap_or ("unnamed snapshot") , ctx . assertion_line) ; } } else { results . insert (key . to_string () , snapshot . clone ()) ; } }
};
}
