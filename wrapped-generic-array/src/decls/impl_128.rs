macro_rules! deps {
    () => {
        ArrayLength!();
        Split!();
        GenericArray!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        unsafe impl < 'a , T , N , K > Split < T , K > for & 'a mut GenericArray < T , N > where N : ArrayLength , K : ArrayLength , N : Sub < K > , Diff < N , K > : ArrayLength , { type First = & 'a mut GenericArray < T , K > ; type Second = & 'a mut GenericArray < T , Diff < N , K > > ; # [inline] fn split (self) -> (Self :: First , Self :: Second) { unsafe { let ptr_to_first : * mut T = self . as_mut_ptr () ; let head = & mut * (ptr_to_first as * mut _) ; let tail = & mut * (ptr_to_first . add (K :: USIZE) as * mut _) ; (head , tail) } } }
    };
}

impl_128!();