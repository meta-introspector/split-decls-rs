macro_rules! deps {
    () => {
        RuleError!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl RuleError { pub (crate) fn new (locations : Vec < Pos > , msg : impl Into < String >) -> Self { Self { locations , message : msg . into () , } } }
    };
}

impl_306!()