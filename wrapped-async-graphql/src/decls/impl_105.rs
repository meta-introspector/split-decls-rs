macro_rules! deps {
    () => {
        BatchRequest!();
        Request!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl From < Request > for BatchRequest { fn from (r : Request) -> Self { BatchRequest :: Single (r) } }
    };
}

impl_105!()