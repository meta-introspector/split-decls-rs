macro_rules! deps {
    () => {
        ArrayVecImpl!();
        ArrayVec!();
        LenUint!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T , const CAP : usize > ArrayVecImpl for ArrayVec < T , CAP > { type Item = T ; const CAPACITY : usize = CAP ; fn len (& self) -> usize { self . len () } unsafe fn set_len (& mut self , length : usize) { debug_assert ! (length <= CAP) ; self . len = length as LenUint ; } fn as_ptr (& self) -> * const Self :: Item { self . xs . as_ptr () as _ } fn as_mut_ptr (& mut self) -> * mut Self :: Item { self . xs . as_mut_ptr () as _ } }
    };
}

impl_41!()