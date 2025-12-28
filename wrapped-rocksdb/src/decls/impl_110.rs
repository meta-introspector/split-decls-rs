macro_rules! deps {
    () => {
        DBCommon!();
        DBInner!();
        ThreadMode!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        unsafe impl < T : ThreadMode , I : DBInner > Sync for DBCommon < T , I > { }
    };
}

impl_110!();