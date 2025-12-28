macro_rules! deps {
    () => {
        Message!();
        Element!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > From < Message < 'a > > for Element < 'a > { fn from (value : Message < 'a >) -> Self { Element :: Message (value) } }
    };
}

impl_121!()