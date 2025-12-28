macro_rules! INITIALIZED {
    () => {
        # [cfg (test)] static INITIALIZED : std :: sync :: atomic :: AtomicBool = std :: sync :: atomic :: AtomicBool :: new (false) ;
    };
}

INITIALIZED!()