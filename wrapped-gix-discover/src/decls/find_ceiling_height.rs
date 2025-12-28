macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! find_ceiling_height {
    () => {
        deps!();
        # [doc = " Find the number of components parenting the `search_dir` before the first directory in `ceiling_dirs`."] # [doc = " `search_dir` needs to be normalized, and we normalize every ceiling as well."] pub (crate) fn find_ceiling_height (search_dir : & Path , ceiling_dirs : & [PathBuf] , cwd : & Path) -> Option < usize > { if ceiling_dirs . is_empty () { return None ; } let search_realpath ; let search_dir = if search_dir . is_absolute () { search_dir } else { search_realpath = gix_path :: realpath_opts (search_dir , cwd , gix_path :: realpath :: MAX_SYMLINKS) . ok () ? ; search_realpath . as_path () } ; ceiling_dirs . iter () . filter_map (| ceiling_dir | { # [cfg (windows)] let ceiling_dir = dunce :: simplified (ceiling_dir) ; let mut ceiling_dir = gix_path :: normalize (ceiling_dir . into () , cwd) ? ; if ! ceiling_dir . is_absolute () { ceiling_dir = gix_path :: normalize (cwd . join (ceiling_dir . as_ref ()) . into () , cwd) ? ; } search_dir . strip_prefix (ceiling_dir . as_ref ()) . ok () . map (| path_relative_to_ceiling | path_relative_to_ceiling . components () . count ()) . filter (| height | * height > 0) }) . min () }
    };
}

find_ceiling_height!();