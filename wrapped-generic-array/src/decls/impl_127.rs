macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
        Split!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        unsafe impl < 'a , T , N , K > Split < T , K > for & 'a GenericArray < T , N > where N : ArrayLength , K : ArrayLength , N : Sub < K > , Diff < N , K > : ArrayLength , { type First = & 'a GenericArray < T , K > ; type Second = & 'a GenericArray < T , Diff < N , K > > ; # [inline] fn split (self) -> (Self :: First , Self :: Second) { unsafe { let ptr_to_first : * const T = self . as_ptr () ; let head = & * (ptr_to_first as * const _) ; let tail = & * (ptr_to_first . add (K :: USIZE) as * const _) ; (head , tail) } } }
    };
}

impl_127!()