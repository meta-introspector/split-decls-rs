macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl < 'a , A : ? Sized + 'static > From < & 'a A > for AnyValueId { fn from (_ : & 'a A) -> Self { Self :: of :: < A > () } }
    };
}

impl_611!()