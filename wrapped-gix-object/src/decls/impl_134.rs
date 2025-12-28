macro_rules! deps {
    () => {
        EntryKind!();
        EntryMode!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl From < EntryKind > for EntryMode { fn from (value : EntryKind) -> Self { EntryMode { internal : value as u16 } } }
    };
}

impl_134!();