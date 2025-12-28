macro_rules! deps {
    () => {
        FieldValue!();
        Request!();
        DynamicRequest!();
        DynamicRequestExt!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < T : Into < Request > > DynamicRequestExt for T { fn root_value (self , value : FieldValue < 'static >) -> DynamicRequest { DynamicRequest { inner : self . into () , root_value : value , } } }
    };
}

impl_459!();