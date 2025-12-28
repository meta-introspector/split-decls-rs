macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! OdbHandleArc {
    () => {
        deps!();
        # [doc = " A handle for finding objects in an object database, abstracting away caches for moving across threads."] pub type OdbHandleArc = gix_odb :: memory :: Proxy < gix_odb :: HandleArc > ;
    };
}

OdbHandleArc!()