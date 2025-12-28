macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : Buf + ? Sized > Buf for & mut T { deref_forward_buf ! () ; }
    };
}

impl_7!();