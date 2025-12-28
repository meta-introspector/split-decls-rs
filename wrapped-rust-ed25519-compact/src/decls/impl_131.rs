macro_rules! deps {
    () => {
        DHOutput!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl DHOutput { pub const BYTES : usize = 32 ; }
    };
}

impl_131!()