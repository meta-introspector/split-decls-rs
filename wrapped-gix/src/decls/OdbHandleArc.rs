macro_rules! OdbHandleArc {
    () => {
        # [doc = " A handle for finding objects in an object database, abstracting away caches for moving across threads."] pub type OdbHandleArc = gix_odb :: memory :: Proxy < gix_odb :: HandleArc > ;
    };
}

OdbHandleArc!()