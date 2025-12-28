macro_rules! deps {
    () => {
        FieldValueInner!();
        FieldValue!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl From < () > for FieldValue < '_ > { # [inline] fn from (_ : ()) -> Self { Self (FieldValueInner :: Value (Value :: Null)) } }
    };
}

impl_426!()