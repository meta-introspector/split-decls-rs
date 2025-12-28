macro_rules! impl_598 {
    () => {
        impl < St : TryStream + FusedStream > FusedStream for IntoStream < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_598!()