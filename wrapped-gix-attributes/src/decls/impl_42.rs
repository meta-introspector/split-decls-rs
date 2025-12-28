macro_rules! deps {
    () => {
        Metadata!();
        AttributeId!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl From < AttributeId > for Metadata { fn from (order : AttributeId) -> Self { Metadata { id : order , macro_attributes : Default :: default () , } } }
    };
}

impl_42!()