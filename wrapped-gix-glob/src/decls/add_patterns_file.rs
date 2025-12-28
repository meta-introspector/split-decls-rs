macro_rules! deps {
    () => {
        List!();
        Pattern!();
    };
}

macro_rules! add_patterns_file {
    () => {
        deps!();
        # [doc = " Add the given file at `source` if it exists, otherwise do nothing."] # [doc = " If a `root` is provided, it's not considered a global file anymore."] # [doc = " `parse` is a way to parse bytes to pattern."] # [doc = " Returns `true` if the file was added, or `false` if it didn't exist."] pub fn add_patterns_file < T : Pattern > (patterns : & mut Vec < pattern :: List < T > > , source : PathBuf , follow_symlinks : bool , root : Option < & Path > , buf : & mut Vec < u8 > , parse : T ,) -> std :: io :: Result < bool > { let previous_len = patterns . len () ; patterns . extend (pattern :: List :: < T > :: from_file (source , root , follow_symlinks , buf , parse ,) ?) ; Ok (patterns . len () != previous_len) }
    };
}

add_patterns_file!();