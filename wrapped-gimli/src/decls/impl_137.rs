macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < A : ArrayLike > ops :: DerefMut for ArrayVec < A > { fn deref_mut (& mut self) -> & mut [A :: Item] { let slice = & mut A :: as_mut_slice (& mut self . storage) ; debug_assert ! (self . len <= slice . len ()) ; unsafe { slice :: from_raw_parts_mut (slice . as_mut_ptr () as _ , self . len) } } }
    };
}

impl_137!();