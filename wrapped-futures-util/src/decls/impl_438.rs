macro_rules! impl_438 {
    () => {
        impl < St : Stream > Take < St > { pub (super) fn new (stream : St , n : usize) -> Self { Self { stream , remaining : n } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_438!()