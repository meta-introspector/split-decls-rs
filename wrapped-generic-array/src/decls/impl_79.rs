macro_rules! deps {
    () => {
        GAVisitor!();
        ArrayLength!();
        GenericArray!();
        IntrusiveArrayBuilder!();
        Dummy!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'de , T , N : ArrayLength > Visitor < 'de > for GAVisitor < T , N > where T : Deserialize < 'de > , { type Value = GenericArray < T , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "struct GenericArray<T, U{}>" , N :: USIZE) } fn visit_seq < A > (self , mut seq : A) -> Result < GenericArray < T , N > , A :: Error > where A : SeqAccess < 'de > , { match seq . size_hint () { Some (n) if n != N :: USIZE => { return Err (de :: Error :: invalid_length (n , & self)) ; } _ => { } } unsafe { let mut dst = core :: mem :: MaybeUninit :: < GenericArray < T , N > > :: uninit () ; let mut builder = IntrusiveArrayBuilder :: new_alt (& mut dst) ; let (build_iter , position) = builder . iter_position () ; for dst in build_iter { match seq . next_element () ? { Some (el) => { dst . write (el) ; * position += 1 ; } None => break , } } if * position == N :: USIZE { if seq . size_hint () != Some (0) && seq . next_element :: < Dummy > () ? . is_some () { return Err (de :: Error :: invalid_length (* position + 1 , & self)) ; } return Ok (builder . finish_and_assume_init ()) ; } Err (de :: Error :: invalid_length (* position , & self)) } } }
    };
}

impl_79!();