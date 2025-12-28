macro_rules! EncodeSliceError {
    () => {
        # [doc = " Errors that can occur while encoding into a slice."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum EncodeSliceError { # [doc = " The provided slice is too small."] OutputSliceTooSmall , }
    };
}

EncodeSliceError!();