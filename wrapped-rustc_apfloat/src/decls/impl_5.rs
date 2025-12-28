macro_rules! deps {
    () => {
        StatusAnd!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > StatusAnd < T > { # [doc = " Keep the existing status but apply a transformation to `value`."] pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> StatusAnd < U > { StatusAnd { status : self . status , value : f (self . value) , } } }
    };
}

impl_5!();