macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        scoped_thread_local ! { static STATE : RefCell < State <'_ >> }
    };
}

macro_136!()