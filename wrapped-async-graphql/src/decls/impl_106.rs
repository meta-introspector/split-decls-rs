macro_rules! deps {
    () => {
        Request!();
        BatchRequest!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl From < Vec < Request > > for BatchRequest { fn from (r : Vec < Request >) -> Self { BatchRequest :: Batch (r) } }
    };
}

impl_106!();