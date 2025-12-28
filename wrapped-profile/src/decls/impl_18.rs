macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl std :: ops :: Sub for Bytes { type Output = Bytes ; fn sub (self , rhs : Bytes) -> Bytes { Bytes (self . 0 - rhs . 0) } }
    };
}

impl_18!();