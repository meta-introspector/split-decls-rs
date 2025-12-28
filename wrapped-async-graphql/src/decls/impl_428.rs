macro_rules! deps {
    () => {
        FieldValue!();
        FieldValueInner!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < 'a , T : Into < FieldValue < 'a > > > From < Vec < T > > for FieldValue < 'a > { fn from (values : Vec < T >) -> Self { Self (FieldValueInner :: List (values . into_iter () . map (Into :: into) . collect () ,)) } }
    };
}

impl_428!();