macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Outcome { # [doc = " The total amount of skipped entries, i.e. those that weren't processed at all."] pub fn skipped (& self) -> usize { self . entries_skipped_by_common_prefix + self . entries_skipped_by_pathspec + self . entries_skipped_by_entry_flags } }
    };
}

impl_4!()