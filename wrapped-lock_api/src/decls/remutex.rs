macro_rules! remutex {
    () => {
        # [cfg (feature = "atomic_usize")] mod remutex ;
    };
}

remutex!();