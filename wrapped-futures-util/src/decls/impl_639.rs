macro_rules! impl_639 {
    () => {
        impl < St > FusedStream for TryFlatten < St > where St : TryStream + FusedStream , St :: Ok : TryStream , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn is_terminated (& self) -> bool { self . next . is_none () && self . stream . is_terminated () } }
    };
}

impl_639!();