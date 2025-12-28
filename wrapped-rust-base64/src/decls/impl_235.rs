macro_rules! deps {
    () => {
        DecodeError!();
        DecodeSliceError!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , test))] impl error :: Error for DecodeSliceError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self { DecodeSliceError :: DecodeError (e) => Some (e) , DecodeSliceError :: OutputSliceTooSmall => None , } } }
    };
}

impl_235!()