macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < A : ArrayLike > ops :: Deref for ArrayVec < A > { type Target = [A :: Item] ; fn deref (& self) -> & [A :: Item] { let slice = & A :: as_slice (& self . storage) ; debug_assert ! (self . len <= slice . len ()) ; unsafe { slice :: from_raw_parts (slice . as_ptr () as _ , self . len) } } }
    };
}

impl_136!();