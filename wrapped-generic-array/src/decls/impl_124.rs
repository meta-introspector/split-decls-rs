macro_rules! deps {
    () => {
        Shorten!();
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        unsafe impl < T , N : ArrayLength > Shorten < T > for GenericArray < T , N > where N : Sub < B1 > , Sub1 < N > : ArrayLength , Sub1 < N > : Add < B1 , Output = N > , Add1 < Sub1 < N > > : ArrayLength , { type Shorter = GenericArray < T , Sub1 < N > > ; # [inline] fn pop_back (self) -> (Self :: Shorter , T) { let whole = ManuallyDrop :: new (self) ; unsafe { let init = ptr :: read (whole . as_ptr () as _) ; let last = ptr :: read (whole . as_ptr () . add (Sub1 :: < N > :: USIZE) as _) ; (init , last) } } # [inline] fn pop_front (self) -> (T , Self :: Shorter) { let whole = ManuallyDrop :: new (self) ; unsafe { let head = ptr :: read (whole . as_ptr () as _) ; let tail = ptr :: read (whole . as_ptr () . offset (1) as _) ; (head , tail) } } }
    };
}

impl_124!();