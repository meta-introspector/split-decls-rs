macro_rules! deps {
    () => {
        DBCommon!();
        DBInner!();
        ThreadMode!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        unsafe impl < T : ThreadMode + Send , I : DBInner > Send for DBCommon < T , I > { }
    };
}

impl_109!()