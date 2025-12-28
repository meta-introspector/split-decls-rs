macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Symbol < '_ , T > { }
    };
}

impl_160!();