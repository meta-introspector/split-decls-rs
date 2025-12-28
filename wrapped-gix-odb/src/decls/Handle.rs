macro_rules! deps {
    () => {
        Store!();
        Cache!();
    };
}

macro_rules! Handle {
    () => {
        deps!();
        # [doc = " A thread-local handle to access any object."] pub type Handle = Cache < store :: Handle < OwnShared < Store > > > ;
    };
}

Handle!()