macro_rules! deps {
    () => {
        InputType!();
        InputValueError!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : InputType , E : Display > From < E > for InputValueError < T > { fn from (error : E) -> Self { Self :: custom (error) } }
    };
}

impl_39!()