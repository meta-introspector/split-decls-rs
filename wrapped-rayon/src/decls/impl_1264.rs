macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_1264 {
    () => {
        deps!();
        impl < T : Debug , P > Debug for Split < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Split") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1264!();