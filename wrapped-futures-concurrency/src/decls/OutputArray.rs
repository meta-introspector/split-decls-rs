macro_rules! OutputArray {
    () => {
        # [doc = " A contiguous array of uninitialized data."] pub (crate) struct OutputArray < T , const N : usize > { data : [MaybeUninit < T > ; N] , }
    };
}

OutputArray!()