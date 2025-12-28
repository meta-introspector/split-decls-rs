macro_rules! deps {
    () => {
        Default!();
        Path!();
        Options!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { location : Some (Location :: Path) , # [cfg (feature = "blob-diff")] rewrites : None , } } }
    };
}

impl_443!()