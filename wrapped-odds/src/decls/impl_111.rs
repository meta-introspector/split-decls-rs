macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'a , A > Stride < 'a , A > { # [doc = " Create a Stride iterator from a raw pointer."] pub unsafe fn from_ptr_len (begin : * const A , nelem : usize , stride : isize) -> Stride < 'a , A > { Stride { begin : begin , offset : 0 , end : stride * nelem as isize , stride : stride , life : marker :: PhantomData , } } }
    };
}

impl_111!()