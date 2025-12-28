macro_rules! LocationSet {
    () => {
        # [derive (Debug)] pub (super) struct LocationSet { locations : [Location ; MAX_THREADS] , }
    };
}

LocationSet!();