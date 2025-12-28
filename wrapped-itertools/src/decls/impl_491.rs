macro_rules! deps {
    () => {
        TakeWhileInclusive!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < I , F > TakeWhileInclusive < I , F > where I : Iterator , F : FnMut (& I :: Item) -> bool , { # [doc = " Create a new [`TakeWhileInclusive`] from an iterator and a predicate."] pub (crate) fn new (iter : I , predicate : F) -> Self { Self { iter , predicate , done : false , } } }
    };
}

impl_491!()