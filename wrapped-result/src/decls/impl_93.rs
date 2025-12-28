macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl From < & bool > for BOOL { fn from (value : & bool) -> Self { (* value) . into () } }
    };
}

impl_93!()