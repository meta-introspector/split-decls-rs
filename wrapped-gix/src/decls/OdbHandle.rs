macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! OdbHandle {
    () => {
        deps!();
        # [doc = " A handle for finding objects in an object database, abstracting away caches for thread-local use."] pub type OdbHandle = gix_odb :: memory :: Proxy < gix_odb :: Handle > ;
    };
}

OdbHandle!();