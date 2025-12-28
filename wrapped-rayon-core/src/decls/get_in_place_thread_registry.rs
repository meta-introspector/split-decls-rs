macro_rules! deps {
    () => {
        WorkerThread!();
        Registry!();
    };
}

macro_rules! get_in_place_thread_registry {
    () => {
        deps!();
        fn get_in_place_thread_registry (registry : Option < & Arc < Registry > > ,) -> (Option < & WorkerThread > , Option < & Arc < Registry > >) { let thread = unsafe { WorkerThread :: current () . as_ref () } ; if thread . is_none () && registry . is_none () { let global = global_registry () ; (global . current_thread () , Some (global)) } else { (thread , registry) } }
    };
}

get_in_place_thread_registry!()