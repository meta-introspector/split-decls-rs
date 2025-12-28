macro_rules! deps {
    () => {
        DemangleWrite!();
        CvQualifiers!();
        DemangleAsInner!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for CvQualifiers where W : 'subs + DemangleWrite { }
    };
}

impl_168!();