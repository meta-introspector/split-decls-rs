macro_rules! deps {
    () => {
        CursorLines!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl CursorLines < '_ > { fn new (src : & str) -> CursorLines < '_ > { CursorLines (src) } }
    };
}

impl_65!()