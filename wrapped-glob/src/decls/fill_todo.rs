macro_rules! deps {
    () => {
        Pattern!();
        PathWrapper!();
        GlobError!();
        MatchOptions!();
        PatternToken!();
    };
}

macro_rules! fill_todo {
    () => {
        deps!();
        fn fill_todo (todo : & mut Vec < Result < (PathWrapper , usize) , GlobError > > , patterns : & [Pattern] , idx : usize , path : & PathWrapper , options : MatchOptions ,) { let add = | todo : & mut Vec < _ > , next_path : PathWrapper | { if idx + 1 == patterns . len () { todo . push (Ok ((next_path , usize :: MAX))) ; } else { fill_todo (todo , patterns , idx + 1 , & next_path , options) ; } } ; let pattern = & patterns [idx] ; let is_dir = path . is_directory ; let curdir = path . as_ref () == Path :: new (".") ; match (pattern . has_metachars , is_dir) { (false , _) => { debug_assert ! (pattern . tokens . iter () . all (| tok | matches ! (tok , PatternToken :: Char (_))) , "broken invariant: pattern has metachars but shouldn't") ; let s = pattern . as_str () ; let special = "." == s || ".." == s ; let next_path = if curdir { PathBuf :: from (s) } else { path . join (s) } ; let next_path = PathWrapper :: from_path (next_path) ; if (special && is_dir) || (! special && (fs :: metadata (& next_path) . is_ok () || fs :: symlink_metadata (& next_path) . is_ok ())) { add (todo , next_path) ; } } (true , true) => { let dirs = fs :: read_dir (path) . and_then (| d | { d . map (| e | { e . map (| e | { let (path , filename) = if curdir { (PathBuf :: from (e . path () . file_name () . unwrap ()) , e . file_name ()) } else { (e . path () , e . file_name ()) } ; (PathWrapper :: from_dir_entry (path , e) , filename) }) }) . collect :: < Result < Vec < (PathWrapper , OsString) > , _ > > () }) ; match dirs { Ok (mut children) => { if options . require_literal_leading_dot { children . retain (| x | ! x . 1 . to_str () . unwrap () . starts_with ('.')) ; } children . sort_by (| p1 , p2 | p2 . 1 . cmp (& p1 . 1)) ; todo . extend (children . into_iter () . map (| x | Ok ((x . 0 , idx)))) ; if ! pattern . tokens . is_empty () && pattern . tokens [0] == Char ('.') { for & special in & ["." , ".."] { if pattern . matches_with (special , options) { add (todo , PathWrapper :: from_path (path . join (special))) ; } } } } Err (e) => { todo . push (Err (GlobError { path : path . to_path_buf () , error : e , })) ; } } } (true , false) => { } } }
    };
}

fill_todo!()