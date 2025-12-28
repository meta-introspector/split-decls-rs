macro_rules! deps {
    () => {
        EncodeSliceError!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl fmt :: Display for EncodeSliceError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: OutputSliceTooSmall => write ! (f , "Output slice too small") , } } }
    };
}

impl_226!();