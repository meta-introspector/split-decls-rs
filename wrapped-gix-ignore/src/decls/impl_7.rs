macro_rules! deps {
    () => {
        Match!();
        Search!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " Matching of ignore patterns."] impl Search { # [doc = " Match `relative_path` and return the first match if found."] # [doc = " `is_dir` is true if `relative_path` is a directory."] # [doc = " `case` specifies whether cases should be folded during matching or not."] pub fn pattern_matching_relative_path (& self , relative_path : & BStr , is_dir : Option < bool > , case : gix_glob :: pattern :: Case ,) -> Option < Match < '_ > > { let basename_pos = relative_path . rfind (b"/") . map (| p | p + 1) ; self . patterns . iter () . rev () . find_map (| pl | pattern_matching_relative_path (pl , relative_path , basename_pos , is_dir , case)) } }
    };
}

impl_7!();