macro_rules! deps {
    () => {
        StringArray!();
        Binding!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl Binding for StringArray { type Raw = raw :: git_strarray ; unsafe fn from_raw (raw : raw :: git_strarray) -> StringArray { StringArray { raw } } fn raw (& self) -> raw :: git_strarray { self . raw } }
    };
}

impl_153!()