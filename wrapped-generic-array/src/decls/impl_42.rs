macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        unsafe impl < T , N : ArrayLength > FallibleGenericSequence < T > for GenericArray < T , N > where Self : IntoIterator < Item = T > , { # [inline (always)] fn try_generate < F , E > (mut f : F) -> Result < Self :: Sequence , E > where F : FnMut (usize) -> Result < T , E > , { unsafe { let mut array = MaybeUninit :: < GenericArray < T , N > > :: uninit () ; let mut builder = IntrusiveArrayBuilder :: new_alt (& mut array) ; let (builder_iter , position) = builder . iter_position () ; if let Err (e) = builder_iter . enumerate () . try_for_each (| (i , dst) | match f (i) { Ok (value) => { dst . write (value) ; * position += 1 ; Ok (()) } Err (e) => Err (e) , }) { drop (builder) ; return Err (e) ; } Ok (builder . finish_and_assume_init ()) } } # [inline (always)] fn from_fallible_iter < I , E > (iter : I) -> Result < Self :: Sequence , E > where I : IntoIterator < Item = Result < T , E > > , { match Self :: try_from_fallible_iter (iter) { Ok (res) => res , Err (_) => from_iter_length_fail (N :: USIZE) , } } }
    };
}

impl_42!()