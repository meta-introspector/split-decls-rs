macro_rules! deps {
    () => {
        LengthError!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T , N : ArrayLength > TryFrom < Box < [T] > > for GenericArray < T , N > { type Error = crate :: LengthError ; # [inline] fn try_from (value : Box < [T] >) -> Result < Self , Self :: Error > { Vec :: from (value) . try_into () } }
    };
}

impl_61!();