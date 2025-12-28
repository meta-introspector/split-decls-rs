macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl From < usize > for ValueRange { fn from (fixed : usize) -> Self { (fixed ..= fixed) . into () } }
    };
}

impl_155!();