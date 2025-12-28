macro_rules! deps {
    () => {
        GenericArray!();
        LengthError!();
        ArrayLength!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a , T , N : ArrayLength > TryFrom < & 'a mut [T] > for & 'a mut GenericArray < T , N > { type Error = LengthError ; # [inline (always)] fn try_from (slice : & 'a mut [T]) -> Result < Self , Self :: Error > { GenericArray :: try_from_mut_slice (slice) } }
    };
}

impl_51!()