macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
        LengthError!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < 'a , T , N : ArrayLength > TryFrom < & 'a [T] > for & 'a GenericArray < T , N > { type Error = LengthError ; # [inline (always)] fn try_from (slice : & 'a [T]) -> Result < Self , Self :: Error > { GenericArray :: try_from_slice (slice) } }
    };
}

impl_196!();