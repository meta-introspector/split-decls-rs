macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! macro_32 {
    () => {
        deps!();
        thread_local ! { static ROOT_CONTEXT_P : Cell <* mut Context > = const { Cell :: new (ptr :: null_mut ()) } ; }
    };
}

macro_32!();