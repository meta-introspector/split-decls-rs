macro_rules! deps {
    () => {
        TargetLoadError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < & str > for TargetLoadError { fn from (value : & str) -> Self { Self (value . into ()) } }
    };
}

impl_71!();