macro_rules! deps {
    () => {
        AnyValue!();
        Result!();
        Error!();
    };
}

macro_rules! impl_601 {
    () => {
        deps!();
        impl std :: fmt :: Debug for AnyValue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("AnyValue") . field ("inner" , & self . id) . finish () } }
    };
}

impl_601!()