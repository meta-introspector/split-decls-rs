macro_rules! DecodeError {
    () => {
        # [doc = " Possible errors in variable-time integer decoding methods."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum DecodeError { # [doc = " The input value was empty."] Empty , # [doc = " The input was not consistent with the format restrictions."] InvalidDigit , # [doc = " Input size is too small to fit in the given precision."] InputSize , # [doc = " The deserialized number is larger than the given precision."] Precision , }
    };
}

DecodeError!()