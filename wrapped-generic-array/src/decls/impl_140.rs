macro_rules! deps {
    () => {
        Unflatten!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        unsafe impl < 'a , T , NM , N > Unflatten < T , NM , N > for & 'a mut GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = & 'a mut GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
    };
}

impl_140!();