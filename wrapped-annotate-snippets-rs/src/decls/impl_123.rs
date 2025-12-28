macro_rules! deps {
    () => {
        Snippet!();
        Patch!();
        Element!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a > From < Snippet < 'a , Patch < 'a > > > for Element < 'a > { fn from (value : Snippet < 'a , Patch < 'a > >) -> Self { Element :: Suggestion (value) } }
    };
}

impl_123!();