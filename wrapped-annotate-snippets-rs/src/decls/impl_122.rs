macro_rules! deps {
    () => {
        Element!();
        Annotation!();
        Snippet!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'a > From < Snippet < 'a , Annotation < 'a > > > for Element < 'a > { fn from (value : Snippet < 'a , Annotation < 'a > >) -> Self { Element :: Cause (value) } }
    };
}

impl_122!()