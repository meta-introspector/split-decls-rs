macro_rules! deps {
    () => {
        EntryMode!();
        EntryKind!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl From < EntryMode > for EntryKind { fn from (value : EntryMode) -> Self { value . kind () } }
    };
}

impl_135!();