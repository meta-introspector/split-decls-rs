macro_rules! deps {
    () => {
        Context!();
        Kind!();
        Outcome!();
        Options!();
        Error!();
    };
}

macro_rules! root {
    () => {
        deps!();
        # [doc = " Classify the `worktree_relative_root` path and return the first `PathKind` that indicates that"] # [doc = " it isn't a directory, leaving `buf` with the path matching the returned `PathKind`,"] # [doc = " which is at most equal to `worktree_relative_root`."] pub fn root (worktree_root : & Path , buf : & mut BString , worktree_relative_root : & Path , options : Options < '_ > , ctx : & mut Context < '_ > ,) -> Result < (Outcome , bool) , Error > { buf . clear () ; let mut last_length = None ; let mut path_buf = worktree_root . to_owned () ; let file_kind = path_buf . symlink_metadata () . ok () . map (| m | m . file_type () . into ()) ; let mut out = path (& mut path_buf , buf , 0 , file_kind , | | None , options , ctx) ? ; let worktree_root_is_repository = out . disk_kind . is_some_and (| kind | matches ! (kind , entry :: Kind :: Repository)) ; for component in worktree_relative_root . components () { if last_length . is_some () { buf . push (b'/') ; } path_buf . push (component) ; buf . extend_from_slice (gix_path :: os_str_into_bstr (component . as_os_str ()) . expect ("no illformed UTF8")) ; let file_kind = path_buf . symlink_metadata () . ok () . map (| m | m . file_type () . into ()) ; out = path (& mut path_buf , buf , last_length . map (| l | l + 1) . unwrap_or_default () , file_kind , | | None , options , ctx ,) ? ; if ! out . status . can_recurse (out . disk_kind , out . pathspec_match , options . for_deletion , worktree_root_is_repository ,) { break ; } last_length = Some (buf . len ()) ; } Ok ((out , worktree_root_is_repository)) }
    };
}

root!()