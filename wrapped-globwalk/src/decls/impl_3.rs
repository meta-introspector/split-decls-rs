macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl From < std :: io :: Error > for GlobError { fn from (e : std :: io :: Error) -> Self { GlobError (e . into ()) } }
    };
}

impl_3!()