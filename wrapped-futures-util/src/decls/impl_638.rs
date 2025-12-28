macro_rules! impl_638 {
    () => {
        impl < St > TryFlatten < St > where St : TryStream , St :: Ok : TryStream , < St :: Ok as TryStream > :: Error : From < St :: Error > , { pub (super) fn new (stream : St) -> Self { Self { stream , next : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_638!();