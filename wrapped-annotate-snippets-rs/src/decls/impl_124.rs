macro_rules! deps {
    () => {
        Element!();
        Origin!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > From < Origin < 'a > > for Element < 'a > { fn from (value : Origin < 'a >) -> Self { Element :: Origin (value) } }
    };
}

impl_124!()