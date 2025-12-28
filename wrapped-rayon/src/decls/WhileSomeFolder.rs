macro_rules! WhileSomeFolder {
    () => {
        struct WhileSomeFolder < 'f , C > { base : C , full : & 'f AtomicBool , }
    };
}

WhileSomeFolder!()