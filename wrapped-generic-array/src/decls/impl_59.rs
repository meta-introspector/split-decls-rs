macro_rules! deps {
    () => {
        ArrayLength!();
        LengthError!();
        GenericArray!();
        IntrusiveArrayBuilder!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T , N : ArrayLength > TryFrom < Vec < T > > for GenericArray < T , N > { type Error = crate :: LengthError ; fn try_from (v : Vec < T >) -> Result < Self , Self :: Error > { if v . len () != N :: USIZE { return Err (crate :: LengthError) ; } unsafe { let mut destination = core :: mem :: MaybeUninit :: < GenericArray < T , N > > :: uninit () ; let mut builder = IntrusiveArrayBuilder :: new_alt (& mut destination) ; builder . extend (v . into_iter ()) ; Ok (builder . finish_and_assume_init ()) } } }
    };
}

impl_59!()