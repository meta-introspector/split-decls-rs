macro_rules! impl_680 {
    () => {
        impl < St : TryStream > TryReadyChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : IntoStream :: new (stream) . fuse () , cap : capacity } } delegate_access_inner ! (stream , St , (. .)) ; }
    };
}

impl_680!()