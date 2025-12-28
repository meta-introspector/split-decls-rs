macro_rules! deps {
    () => {
        ChunkIndex!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl ChunkIndex { # [inline (always)] fn new (size : usize) -> Self { Self { size , index : 0 , key : 0 , } } }
    };
}

impl_254!();