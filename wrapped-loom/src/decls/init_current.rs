macro_rules! deps {
    () => {
        Execution!();
        Thread!();
        ThreadId!();
    };
}

macro_rules! init_current {
    () => {
        deps!();
        fn init_current (execution : & mut Execution , name : Option < String >) -> Thread { let id = execution . threads . active_id () ; let thread = Thread { id : ThreadId { id } , name , } ; execution . threads . local_init (& CURRENT_THREAD_KEY , thread . clone ()) ; thread }
    };
}

init_current!();