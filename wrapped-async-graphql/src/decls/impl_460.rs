macro_rules! deps {
    () => {
        Request!();
        DynamicRequest!();
        FieldValue!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < T : Into < Request > > From < T > for DynamicRequest { fn from (req : T) -> Self { Self { inner : req . into () , root_value : FieldValue :: NULL , } } }
    };
}

impl_460!()