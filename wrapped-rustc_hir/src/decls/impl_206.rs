macro_rules! deps {
    () => {
        CoroutineKind!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl fmt :: Display for CoroutineKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineKind :: Desugared (d , k) => { d . fmt (f) ? ; k . fmt (f) } CoroutineKind :: Coroutine (_) => f . write_str ("coroutine") , } } }
    };
}

impl_206!();