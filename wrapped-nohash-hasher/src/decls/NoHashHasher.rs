macro_rules! NoHashHasher {
    () => {
        # [cfg (not (debug_assertions))] pub struct NoHashHasher < T > (u64 , PhantomData < T >) ;
    };
}

NoHashHasher!();