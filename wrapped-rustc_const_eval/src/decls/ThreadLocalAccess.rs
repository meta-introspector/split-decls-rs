macro_rules! ThreadLocalAccess {
    () => {
        # [doc = " An access to a thread-local `static`."] # [derive (Debug)] pub (crate) struct ThreadLocalAccess ;
    };
}

ThreadLocalAccess!();