macro_rules! deps {
    () => {
        Attributes!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Attributes { fn may_use_glob_pattern (pattern : & gix_glob :: Pattern) -> bool { pattern . mode != macro_mode () } }
    };
}

impl_30!();