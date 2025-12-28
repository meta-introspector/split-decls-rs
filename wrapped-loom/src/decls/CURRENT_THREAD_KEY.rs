macro_rules! deps {
    () => {
        LocalKey!();
        Thread!();
    };
}

macro_rules! CURRENT_THREAD_KEY {
    () => {
        deps!();
        static CURRENT_THREAD_KEY : LocalKey < Thread > = LocalKey { init : | | unreachable ! () , _p : PhantomData , } ;
    };
}

CURRENT_THREAD_KEY!();