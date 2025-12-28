macro_rules! deps {
    () => {
        AttributePlace!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl AttributePlace { pub (crate) fn as_uint (self) -> c_uint { match self { AttributePlace :: ReturnValue => 0 , AttributePlace :: Argument (i) => 1 + i , AttributePlace :: Function => ! 0 , } } }
    };
}

impl_508!()