macro_rules! deps {
    () => {
        Tree!();
        Init!();
        Any!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl Init { # [doc = " The `init.defaultBranch` key."] pub const DEFAULT_BRANCH : keys :: Any = keys :: Any :: new ("defaultBranch" , & config :: Tree :: INIT) . with_deviation ("If not set, we use `main` instead of `master`") ; }
    };
}

impl_680!();