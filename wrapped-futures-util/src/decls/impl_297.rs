macro_rules! impl_297 {
    () => {
        impl < St > Concat < St > where St : Stream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { pub (super) fn new (stream : St) -> Self { Self { stream , accum : None } } }
    };
}

impl_297!();