macro_rules! deps {
    () => {
        Binding!();
        IndexTime!();
    };
}

macro_rules! impl_792 {
    () => {
        deps!();
        impl Binding for IndexTime { type Raw = raw :: git_index_time ; unsafe fn from_raw (raw : raw :: git_index_time) -> IndexTime { IndexTime { raw } } fn raw (& self) -> raw :: git_index_time { self . raw } }
    };
}

impl_792!()