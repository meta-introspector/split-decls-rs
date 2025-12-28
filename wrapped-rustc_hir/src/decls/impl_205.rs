macro_rules! deps {
    () => {
        CoroutineDesugaring!();
        CoroutineSource!();
        Closure!();
        CoroutineKind!();
        Block!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl CoroutineKind { pub fn movability (self) -> Movability { match self { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _) | CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _) => Movability :: Static , CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => Movability :: Movable , CoroutineKind :: Coroutine (mov) => mov , } } pub fn is_fn_like (self) -> bool { matches ! (self , CoroutineKind :: Desugared (_ , CoroutineSource :: Fn)) } pub fn to_plural_string (& self) -> String { match self { CoroutineKind :: Desugared (d , CoroutineSource :: Fn) => format ! ("{d:#}fn bodies") , CoroutineKind :: Desugared (d , CoroutineSource :: Block) => format ! ("{d:#}blocks") , CoroutineKind :: Desugared (d , CoroutineSource :: Closure) => format ! ("{d:#}closure bodies") , CoroutineKind :: Coroutine (_) => "coroutines" . to_string () , } } }
    };
}

impl_205!()