macro_rules! deps {
    () => {
        ThreadIndices!();
        OnceLock!();
    };
}

macro_rules! thread_indices {
    () => {
        deps!();
        fn thread_indices () -> & 'static Mutex < ThreadIndices > { static THREAD_INDICES : OnceLock < Mutex < ThreadIndices > > = OnceLock :: new () ; fn init () -> Mutex < ThreadIndices > { Mutex :: new (ThreadIndices { mapping : HashMap :: new () , free_list : Vec :: new () , next_index : 0 , }) } THREAD_INDICES . get_or_init (init) }
    };
}

thread_indices!()