macro_rules! deps {
    () => {
        Registry!();
        Scope!();
    };
}

macro_rules! do_in_place_scope {
    () => {
        deps!();
        pub (crate) fn do_in_place_scope < 'scope , OP , R > (registry : Option < & Arc < Registry > > , op : OP) -> R where OP : FnOnce (& Scope < 'scope >) -> R , { let (thread , registry) = get_in_place_thread_registry (registry) ; let scope = Scope :: < 'scope > :: new (thread , registry) ; scope . base . complete (thread , | | op (& scope)) }
    };
}

do_in_place_scope!();