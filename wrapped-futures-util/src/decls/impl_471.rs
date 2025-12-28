macro_rules! impl_471 {
    () => {
        impl < St , Fut , T , F > TryFold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : TryFuture < Ok = T > , { pub (super) fn new (stream : St , f : F , t : T) -> Self { Self { stream , f , accum : Some (t) , future : None } } }
    };
}

impl_471!();