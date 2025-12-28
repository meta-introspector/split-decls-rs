macro_rules! deps {
    () => {
        BufMut!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        unsafe impl < T : BufMut + ? Sized > BufMut for Box < T > { deref_forward_bufmut ! () ; }
    };
}

impl_16!();