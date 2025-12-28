macro_rules! deps {
    () => {
        Transfer!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'easy , 'data > fmt :: Debug for Transfer < 'easy , 'data > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Transfer") . field ("easy" , & self . easy) . finish () } }
    };
}

impl_54!();