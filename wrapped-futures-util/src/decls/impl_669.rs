macro_rules! impl_669 {
    () => {
        impl < St : TryStream > TryChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : IntoStream :: new (stream) . fuse () , items : Vec :: with_capacity (capacity) , cap : capacity , } } fn take (self : Pin < & mut Self >) -> Vec < St :: Ok > { let cap = self . cap ; mem :: replace (self . project () . items , Vec :: with_capacity (cap)) } delegate_access_inner ! (stream , St , (. .)) ; }
    };
}

impl_669!();