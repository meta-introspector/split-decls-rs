macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_575 {
    () => {
        deps!();
        impl < St , Fut , F , E > TryForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = Result < () , E > > , { pub (super) fn new (stream : St , limit : Option < usize > , f : F) -> Self { Self { stream : Some (stream) , limit : limit . and_then (NonZeroUsize :: new) , f , futures : FuturesUnordered :: new () , } } }
    };
}

impl_575!();