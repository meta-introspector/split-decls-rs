macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for Symbol < T > { }
    };
}

impl_99!()