macro_rules! deps {
    () => {
        BufMut!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl < T : BufMut + ? Sized > BufMut for & mut T { deref_forward_bufmut ! () ; }
    };
}

impl_15!();