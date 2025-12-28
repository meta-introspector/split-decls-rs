macro_rules! deps {
    () => {
        StrideMut!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'a , A > StrideMut < 'a , A > { # [doc = " Create a StrideMut iterator from a raw pointer."] pub unsafe fn from_ptr_len (begin : * mut A , nelem : usize , stride : isize) -> StrideMut < 'a , A > { StrideMut { begin : begin , offset : 0 , end : stride * nelem as isize , stride : stride , life : marker :: PhantomData , } } }
    };
}

impl_112!()