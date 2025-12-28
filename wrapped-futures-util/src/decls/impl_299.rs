macro_rules! impl_299 {
    () => {
        impl < St > FusedFuture for Concat < St > where St : FusedStream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . stream . is_terminated () } }
    };
}

impl_299!();