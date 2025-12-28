macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArrayIter!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T , N : ArrayLength > GenericArrayIter < T , N > { # [doc = " Returns the remaining items of this iterator as a slice"] # [inline (always)] pub fn as_slice (& self) -> & [T] { unsafe { self . array . get_unchecked (self . index .. self . index_back) } } # [doc = " Returns the remaining items of this iterator as a mutable slice"] # [inline (always)] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { self . array . get_unchecked_mut (self . index .. self . index_back) } } }
    };
}

impl_48!()