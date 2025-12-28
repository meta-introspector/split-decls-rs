macro_rules! DecodeStep {
    () => {
        # [doc = " Constant-time decoder step."] # [derive (Debug)] pub enum DecodeStep { # [doc = " Match the given range, offsetting the input on match."] Range (RangeInclusive < u8 > , i16) , # [doc = " Match the given value, returning the associated offset on match."] Eq (u8 , i16) , }
    };
}

DecodeStep!();