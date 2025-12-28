macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        impl < St , Fut , F > ForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { pub (super) fn new (stream : St , limit : Option < usize > , f : F) -> Self { Self { stream : Some (stream) , limit : limit . and_then (NonZeroUsize :: new) , f , futures : FuturesUnordered :: new () , } } }
    };
}

impl_549!();