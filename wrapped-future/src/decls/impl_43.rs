macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl AsyncStatus { pub const Canceled : Self = Self (2i32) ; pub const Completed : Self = Self (1i32) ; pub const Error : Self = Self (3i32) ; pub const Started : Self = Self (0i32) ; }
    };
}

impl_43!()