macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleAsInner!();
        CvQualifiers!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for CvQualifiers where W : 'subs + DemangleWrite { }
    };
}

impl_168!()