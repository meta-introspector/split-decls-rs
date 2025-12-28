macro_rules! impl_487 {
    () => {
        impl < St : Stream > ReadyChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : stream . fuse () , cap : capacity } } delegate_access_inner ! (stream , St , (.)) ; }
    };
}

impl_487!()