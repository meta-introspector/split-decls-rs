macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Symbol < T > { }
    };
}

impl_98!();