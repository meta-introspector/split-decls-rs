macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! DecodeSliceError {
    () => {
        deps!();
        # [doc = " Errors that can occur while decoding into a slice."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum DecodeSliceError { # [doc = " A [`DecodeError`] occurred"] DecodeError (DecodeError) , # [doc = " The provided slice is too small."] OutputSliceTooSmall , }
    };
}

DecodeSliceError!()