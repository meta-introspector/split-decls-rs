macro_rules! impl_645 {
    () => {
        impl < St > NestedTryStreamIntoEitherTryStream < St > where St : TryStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn new (stream : St) -> Self { Self { stream } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_645!();