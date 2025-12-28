macro_rules! AtomicGeneration {
    () => {
        pub (crate) type AtomicGeneration = AtomicU32 ;
    };
}

AtomicGeneration!()