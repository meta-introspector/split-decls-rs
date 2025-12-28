macro_rules! deps {
    () => {
        FallibleGenericSequence!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        unsafe impl < 'a , T : 'a , S : FallibleGenericSequence < T > > FallibleGenericSequence < T > for & 'a S where & 'a S : IntoIterator , { # [inline (always)] fn try_generate < F , E > (f : F) -> Result < Self :: Sequence , E > where F : FnMut (usize) -> Result < T , E > , { S :: try_generate (f) } # [inline (always)] fn from_fallible_iter < I , E > (iter : I) -> Result < Self :: Sequence , E > where I : IntoIterator < Item = Result < T , E > > , { S :: from_fallible_iter (iter) } }
    };
}

impl_118!()