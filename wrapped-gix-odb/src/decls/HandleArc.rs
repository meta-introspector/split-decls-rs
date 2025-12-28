macro_rules! deps {
    () => {
        Handle!();
        Cache!();
        Store!();
    };
}

macro_rules! HandleArc {
    () => {
        deps!();
        # [doc = " A thread-local handle to access any object, but thread-safe and independent of the actual type of `OwnShared` or feature toggles in `gix-features`."] pub type HandleArc = Cache < store :: Handle < Arc < Store > > > ;
    };
}

HandleArc!();