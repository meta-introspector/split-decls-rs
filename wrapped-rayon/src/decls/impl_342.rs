macro_rules! deps {
    () => {
        SendPtr!();
        CollectConsumer!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'c , T : Send + 'c > CollectConsumer < 'c , T > { # [doc = " The target memory is considered uninitialized, and will be"] # [doc = " overwritten without reading or dropping existing values."] unsafe fn new (start : * mut T , len : usize) -> Self { CollectConsumer { start : SendPtr (start) , len , marker : PhantomData , } } }
    };
}

impl_342!()