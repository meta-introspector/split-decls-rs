macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T : Buf + ? Sized > Buf for Box < T > { deref_forward_buf ! () ; }
    };
}

impl_8!();