macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl From < Label > for String { fn from (label : Label) -> String { label . 0 } }
    };
}

impl_86!()