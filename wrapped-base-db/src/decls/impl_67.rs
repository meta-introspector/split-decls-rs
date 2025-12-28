macro_rules! deps {
    () => {
        TargetLoadError!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl fmt :: Debug for TargetLoadError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_67!()