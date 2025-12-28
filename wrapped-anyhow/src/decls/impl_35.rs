macro_rules! deps {
    () => {
        BothDebug!();
        Error!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < A , B > BothDebug for (A , B) where A : Debug , B : Debug , { fn __dispatch_ensure (self , msg : & 'static str) -> Error { render (msg , & self . 0 , & self . 1) } }
    };
}

impl_35!();