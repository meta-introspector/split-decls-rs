macro_rules! deps {
    () => {
        Module!();
        VTab!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        unsafe impl < 'vtab , T : VTab < 'vtab > > Sync for Module < 'vtab , T > { }
    };
}

impl_531!()