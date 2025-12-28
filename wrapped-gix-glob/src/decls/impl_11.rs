macro_rules! deps {
    () => {
        Pattern!();
        List!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl < T > List < T > where T : Pattern , { # [doc = " `source_file` is the location of the `bytes` which represents a list of patterns, one pattern per line."] # [doc = " If `root` is `Some(…)` it's used to see `source_file` as relative to itself, if `source_file` is absolute."] # [doc = " If source is relative and should be treated as base, set `root` to `Some(\"\")`."] # [doc = " `parse` is a way to parse bytes to pattern."] pub fn from_bytes (bytes : & [u8] , source_file : PathBuf , root : Option < & Path > , parse : T) -> Self { let patterns = parse . bytes_to_patterns (bytes , source_file . as_path ()) ; let base = root . and_then (| root | source_file . parent () . expect ("file") . strip_prefix (root) . ok ()) . and_then (| base | { (! base . as_os_str () . is_empty ()) . then (| | { let mut base : BString = gix_path :: to_unix_separators_on_windows (gix_path :: into_bstr (base)) . into_owned () ; base . push_byte (b'/') ; base }) }) ; List { patterns , source : Some (source_file) , base , } } # [doc = " Create a pattern list from the `source` file, which may be located underneath `root`, while optionally"] # [doc = " following symlinks with `follow_symlinks`, providing `buf` to temporarily store the data contained in the file."] # [doc = " `parse` is a way to parse bytes to pattern."] pub fn from_file (source : impl Into < PathBuf > , root : Option < & Path > , follow_symlinks : bool , buf : & mut Vec < u8 > , parse : T ,) -> std :: io :: Result < Option < Self > > { let source = source . into () ; Ok (read_in_full_ignore_missing (& source , follow_symlinks , buf) ? . then (| | Self :: from_bytes (buf , source , root , parse))) } }
    };
}

impl_11!()