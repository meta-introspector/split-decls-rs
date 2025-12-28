macro_rules! deps {
    () => {
        PackCache!();
    };
}

macro_rules! NewPackCacheFn {
    () => {
        deps!();
        # [doc = " A constructor for boxed pack caches."] pub type NewPackCacheFn = dyn Fn () -> Box < PackCache > + Send + Sync + 'static ;
    };
}

NewPackCacheFn!()