macro_rules! deps {
    () => {
        FieldValue!();
        FieldValueInner!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl From < Value > for FieldValue < '_ > { # [inline] fn from (value : Value) -> Self { Self (FieldValueInner :: Value (value)) } }
    };
}

impl_427!()