macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        # [cfg (feature = "blob-diff")] impl From < Options > for gix_diff :: tree_with_rewrites :: Options { fn from (opts : Options) -> Self { gix_diff :: tree_with_rewrites :: Options { location : opts . location , # [cfg (feature = "blob-diff")] rewrites : opts . rewrites , } } }
    };
}

impl_444!();