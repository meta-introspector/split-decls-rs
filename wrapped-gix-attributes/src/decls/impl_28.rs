macro_rules! deps {
    () => {
        Outcome!();
        Search!();
        Match!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [doc = " Access and matching"] impl Search { # [doc = " Match `relative_path`, a path relative to the repository, while respective `case`-sensitivity and write them to `out`"] # [doc = " Return `true` if at least one pattern matched."] pub fn pattern_matching_relative_path (& self , relative_path : & BStr , case : gix_glob :: pattern :: Case , is_dir : Option < bool > , out : & mut Outcome ,) -> bool { let basename_pos = relative_path . rfind (b"/") . map (| p | p + 1) ; let mut has_match = false ; self . patterns . iter () . rev () . any (| pl | { has_match |= pattern_matching_relative_path (pl , relative_path , basename_pos , case , is_dir , out) ; out . is_done () }) ; has_match } # [doc = " Return the amount of pattern lists contained in this instance."] pub fn num_pattern_lists (& self) -> usize { self . patterns . len () } }
    };
}

impl_28!()