macro_rules! deps {
    () => {
        ObjectCache!();
    };
}

macro_rules! NewObjectCacheFn {
    () => {
        deps!();
        # [doc = " A constructor for boxed object caches."] pub type NewObjectCacheFn = dyn Fn () -> Box < ObjectCache > + Send + Sync + 'static ;
    };
}

NewObjectCacheFn!()