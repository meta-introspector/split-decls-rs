macro_rules! deps {
    () => {
        Form!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl fmt :: Debug for Form { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Form") . field ("fields" , & "...") . finish () } }
    };
}

impl_39!();