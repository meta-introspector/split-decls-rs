macro_rules! deps {
    () => {
        Module!();
        VTab!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        unsafe impl < 'vtab , T : VTab < 'vtab > > Send for Module < 'vtab , T > { }
    };
}

impl_530!();