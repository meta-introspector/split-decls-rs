macro_rules! impl_656 {
    () => {
        impl < St > FusedStream for NestedTryStreamIntoEitherTryStream < St > where St : TryStream + FusedStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_656!()