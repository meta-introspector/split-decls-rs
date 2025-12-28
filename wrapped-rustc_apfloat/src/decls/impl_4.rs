macro_rules! deps {
    () => {
        StatusAnd!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Status { # [doc = " Add a value to this status to create a [`StatusAnd`]."] pub fn and < T > (self , value : T) -> StatusAnd < T > { StatusAnd { status : self , value } } }
    };
}

impl_4!();