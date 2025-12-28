macro_rules! OdbHandle {
    () => {
        # [doc = " A handle for finding objects in an object database, abstracting away caches for thread-local use."] pub type OdbHandle = gix_odb :: memory :: Proxy < gix_odb :: Handle > ;
    };
}

OdbHandle!()