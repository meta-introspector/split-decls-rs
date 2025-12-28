macro_rules! impl_606 {
    () => {
        impl < St , Fut , F > OrElse < St , Fut , F > where St : TryStream , F : FnMut (St :: Error) -> Fut , Fut : TryFuture < Ok = St :: Ok > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_606!()