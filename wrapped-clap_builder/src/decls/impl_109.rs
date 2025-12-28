macro_rules! deps {
    () => {
        Str!();
        OsStr!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl From < & '_ Str > for OsStr { fn from (id : & '_ Str) -> Self { id . clone () . into () } }
    };
}

impl_109!()