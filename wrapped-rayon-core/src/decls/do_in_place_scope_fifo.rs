macro_rules! deps {
    () => {
        Registry!();
        ScopeFifo!();
    };
}

macro_rules! do_in_place_scope_fifo {
    () => {
        deps!();
        pub (crate) fn do_in_place_scope_fifo < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& ScopeFifo < 'scope >) -> R , { let (thread , registry) = get_in_place_thread_registry (registry) ; let scope = ScopeFifo :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }
    };
}

do_in_place_scope_fifo!();