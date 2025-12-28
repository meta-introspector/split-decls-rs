macro_rules! deps {
    () => {
        GenericSequence!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        unsafe impl < 'a , T : 'a , S : GenericSequence < T > > GenericSequence < T > for & 'a mut S where & 'a mut S : IntoIterator , { type Length = S :: Length ; type Sequence = S :: Sequence ; # [inline (always)] fn generate < F > (f : F) -> Self :: Sequence where F : FnMut (usize) -> T , { S :: generate (f) } }
    };
}

impl_119!();