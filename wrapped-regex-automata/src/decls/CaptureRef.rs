macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! CaptureRef {
    () => {
        deps!();
        # [doc = " `CaptureRef` represents a reference to a capture group inside some text."] # [doc = " The reference is either a capture group name or a number."] # [doc = ""] # [doc = " It is also tagged with the position in the text following the"] # [doc = " capture reference."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] struct CaptureRef < 'a > { cap : Ref < 'a > , end : usize , }
    };
}

CaptureRef!();