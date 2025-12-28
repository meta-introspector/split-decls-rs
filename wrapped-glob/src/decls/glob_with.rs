macro_rules! deps {
    () => {
        PatternError!();
        MatchOptions!();
        Pattern!();
        PathWrapper!();
        Paths!();
    };
}

macro_rules! glob_with {
    () => {
        deps!();
        # [doc = " Return an iterator that produces all the `Path`s that match the given"] # [doc = " pattern using the specified match options, which may be absolute or relative"] # [doc = " to the current working directory."] # [doc = ""] # [doc = " This may return an error if the pattern is invalid."] # [doc = ""] # [doc = " This function accepts Unix shell style patterns as described by"] # [doc = " `Pattern::new(..)`.  The options given are passed through unchanged to"] # [doc = " `Pattern::matches_with(..)` with the exception that"] # [doc = " `require_literal_separator` is always set to `true` regardless of the value"] # [doc = " passed to this function."] # [doc = ""] # [doc = " Paths are yielded in alphabetical order."] pub fn glob_with (pattern : & str , options : MatchOptions) -> Result < Paths , PatternError > { # [cfg (windows)] fn check_windows_verbatim (p : & Path) -> bool { match p . components () . next () { Some (Component :: Prefix (ref p)) => { p . kind () . is_verbatim () && if let std :: path :: Prefix :: VerbatimDisk (_) = p . kind () { false } else { true } } _ => false , } } # [cfg (not (windows))] fn check_windows_verbatim (_ : & Path) -> bool { false } # [cfg (windows)] fn to_scope (p : & Path) -> PathBuf { p . to_path_buf () } # [cfg (not (windows))] fn to_scope (p : & Path) -> PathBuf { p . to_path_buf () } let _ = Pattern :: new (pattern) ? ; let mut components = Path :: new (pattern) . components () . peekable () ; loop { match components . peek () { Some (& Component :: Prefix (..)) | Some (& Component :: RootDir) => { components . next () ; } _ => break , } } let rest = components . map (| s | s . as_os_str ()) . collect :: < PathBuf > () ; let normalized_pattern = Path :: new (pattern) . iter () . collect :: < PathBuf > () ; let root_len = normalized_pattern . to_str () . unwrap () . len () - rest . to_str () . unwrap () . len () ; let root = if root_len > 0 { Some (Path :: new (& pattern [.. root_len])) } else { None } ; if root_len > 0 && check_windows_verbatim (root . unwrap ()) { return Ok (Paths { dir_patterns : Vec :: new () , require_dir : false , options , todo : Vec :: new () , scope : None , }) ; } let scope = root . map_or_else (| | PathBuf :: from (".") , to_scope) ; let scope = PathWrapper :: from_path (scope) ; let mut dir_patterns = Vec :: new () ; let components = pattern [cmp :: min (root_len , pattern . len ()) ..] . split_terminator (path :: is_separator) ; for component in components { dir_patterns . push (Pattern :: new (component) ?) ; } if root_len == pattern . len () { dir_patterns . push (Pattern { original : "" . to_string () , tokens : Vec :: new () , is_recursive : false , has_metachars : false , }) ; } let last_is_separator = pattern . chars () . next_back () . map (path :: is_separator) ; let require_dir = last_is_separator == Some (true) ; let todo = Vec :: new () ; Ok (Paths { dir_patterns , require_dir , options , todo , scope : Some (scope) , }) }
    };
}

glob_with!()