macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for Symbol < '_ , T > { }
    };
}

impl_161!();