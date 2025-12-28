macro_rules! impl_425 {
    () => {
        impl < St : Stream > Skip < St > { pub (super) fn new (stream : St , n : usize) -> Self { Self { stream , remaining : n } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_425!()