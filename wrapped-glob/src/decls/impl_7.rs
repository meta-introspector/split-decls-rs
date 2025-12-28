macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl From < GlobError > for io :: Error { fn from (value : GlobError) -> Self { value . error } }
    };
}

impl_7!();