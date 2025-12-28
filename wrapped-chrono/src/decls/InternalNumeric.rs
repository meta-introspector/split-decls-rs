macro_rules! deps {
    () => {
        Void!();
    };
}

macro_rules! InternalNumeric {
    () => {
        deps!();
        # [doc = " An opaque type representing numeric item types for internal uses only."] # [derive (Clone , Eq , Hash , PartialEq)] pub struct InternalNumeric { _dummy : Void , }
    };
}

InternalNumeric!();