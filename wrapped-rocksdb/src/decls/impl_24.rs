macro_rules! deps {
    () => {
        BackupEngine!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        unsafe impl Send for BackupEngine { }
    };
}

impl_24!();