// Generated macro for find_pending_snapshots (function)
macro_rules! Depcrate_walkfind_pending_snapshots {
() => {
// Module: crate::walk
// Provides: {"find_pending_snapshots"}
// Dependencies: {}
# [doc = " Finds all pending snapshots"] pub (crate) fn find_pending_snapshots < 'a > (package_root : & Path , extensions : & 'a [& 'a str] , flags : FindFlags ,) -> impl Iterator < Item = Result < SnapshotContainer , Box < dyn Error > > > + 'a { make_snapshot_walker (package_root , extensions , flags) . filter_map (Result :: ok) . filter_map (| entry | { let fname = entry . file_name () . to_string_lossy () ; let path = entry . clone () . into_path () ; # [allow (clippy :: manual_map)] if let Some (new_fname) = fname . strip_suffix (".new") { Some (SnapshotContainer :: load (path . clone () , path . with_file_name (new_fname) , TextSnapshotKind :: File ,)) } else if let Some (new_fname) = fname . strip_prefix ('.') . and_then (| f | f . strip_suffix (".pending-snap")) { Some (SnapshotContainer :: load (path . clone () , path . with_file_name (new_fname) , TextSnapshotKind :: Inline ,)) } else { None } }) }
};
}
