macro_rules! deps {
    () => {
        Multi!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl fmt :: Debug for Multi { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Multi") . field ("raw" , & self . raw) . finish () } }
    };
}

impl_134!()