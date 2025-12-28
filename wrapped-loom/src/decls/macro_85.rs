macro_rules! deps {
    () => {
        Notify!();
        Channel!();
        RwLock!();
        State!();
        Mutex!();
        Atomic!();
        Arc!();
        Condvar!();
        Cell!();
    };
}

macro_rules! macro_85 {
    () => {
        deps!();
        objects ! { # [derive (Debug)] # [allow (clippy :: large_enum_variant)] Entry , Alloc (rt :: alloc :: State) , Arc (rt :: arc :: State) , Atomic (rt :: atomic :: State) , Mutex (rt :: mutex :: State) , Condvar (rt :: condvar :: State) , Notify (rt :: notify :: State) , RwLock (rt :: rwlock :: State) , Channel (rt :: mpsc :: State) , Cell (rt :: cell :: State) , }
    };
}

macro_85!()