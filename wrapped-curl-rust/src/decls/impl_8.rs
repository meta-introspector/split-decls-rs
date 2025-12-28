macro_rules! deps {
    () => {
        ShareError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl fmt :: Debug for ShareError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ShareError {{ description: {:?}, code: {} }}" , self . description () , self . code) } }
    };
}

impl_8!()