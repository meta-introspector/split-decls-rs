macro_rules! deps {
    () => {
        Result!();
        Field!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl Debug for Field { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Field") . field ("name" , & self . name) . field ("description" , & self . description) . field ("arguments" , & self . arguments) . field ("ty" , & self . ty) . field ("deprecation" , & self . deprecation) . finish () } }
    };
}

impl_437!()