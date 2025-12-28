macro_rules! macro_39 {
    () => {
        thread_local ! { static DROP_COUNTER : Cell < u32 > = Cell :: new (0) }
    };
}

macro_39!();