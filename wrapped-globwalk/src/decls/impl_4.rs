macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl From < GlobError > for std :: io :: Error { fn from (e : GlobError) -> Self { if let ignore :: Error :: Io (e) = e . 0 { e } else { std :: io :: ErrorKind :: Other . into () } } }
    };
}

impl_4!()