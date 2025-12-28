macro_rules! deps {
    () => {
        UnsafeLint!();
    };
}

macro_rules! MissingUnsafe {
    () => {
        deps!();
        # [derive (Debug)] pub struct MissingUnsafe { pub node : InFile < ExprOrPatPtr > , pub lint : UnsafeLint , pub reason : UnsafetyReason , }
    };
}

MissingUnsafe!();