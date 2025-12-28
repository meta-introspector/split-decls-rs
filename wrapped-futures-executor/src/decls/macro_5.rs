macro_rules! deps {
    () => {
        ThreadNotify!();
    };
}

macro_rules! macro_5 {
    () => {
        deps!();
        std :: thread_local ! { static CURRENT_THREAD_NOTIFY : Arc < ThreadNotify > = Arc :: new (ThreadNotify { thread : thread :: current () , unparked : AtomicBool :: new (false) , }) ; }
    };
}

macro_5!()