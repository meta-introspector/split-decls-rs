macro_rules! ArcInner {
    () => {
        # [doc = " The object allocated by an Arc<T>"] # [repr (C)] pub (crate) struct ArcInner < T : ? Sized > { pub (crate) count : atomic :: AtomicUsize , pub (crate) data : T , }
    };
}

ArcInner!()