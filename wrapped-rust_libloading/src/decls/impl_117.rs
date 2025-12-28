macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Symbol < T > { }
    };
}

impl_117!()