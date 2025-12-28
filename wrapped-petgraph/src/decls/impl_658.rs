macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl < Ix : fmt :: Debug > fmt :: Debug for NodeIndex < Ix > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "NodeIndex({:?})" , self . 0) } }
    };
}

impl_658!();