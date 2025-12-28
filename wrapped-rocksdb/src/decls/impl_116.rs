macro_rules! deps {
    () => {
        DBCommon!();
        ThreadMode!();
        DBInner!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < T : ThreadMode , I : DBInner > Drop for DBCommon < T , I > { fn drop (& mut self) { self . cfs . drop_all_cfs_internal () ; } }
    };
}

impl_116!();