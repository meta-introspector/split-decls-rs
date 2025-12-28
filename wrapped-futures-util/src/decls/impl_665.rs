macro_rules! impl_665 {
    () => {
        impl < St > TryConcat < St > where St : TryStream , St :: Ok : Extend < < St :: Ok as IntoIterator > :: Item > + IntoIterator + Default , { pub (super) fn new (stream : St) -> Self { Self { stream , accum : None } } }
    };
}

impl_665!();