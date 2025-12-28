macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Source { # [doc = " Returns non-worktree variants of `self` if `is_bare` is true."] pub fn adjust_for_bare (self , is_bare : bool) -> Self { if is_bare { Source :: IdMapping } else { self } } }
    };
}

impl_31!();