macro_rules! deps {
    () => {
        Origin!();
        Element!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > From < Origin < 'a > > for Element < 'a > { fn from (value : Origin < 'a >) -> Self { Element :: Origin (value) } }
    };
}

impl_124!();