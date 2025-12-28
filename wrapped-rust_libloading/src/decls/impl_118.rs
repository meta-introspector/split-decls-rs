macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for Symbol < T > { }
    };
}

impl_118!();