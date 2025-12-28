macro_rules! deps {
    () => {
        JobRef!();
        PanicHandler!();
        ExitHandler!();
        StartHandler!();
        ThreadInfo!();
        Sleep!();
    };
}

macro_rules! Registry {
    () => {
        deps!();
        pub (super) struct Registry { thread_infos : Vec < ThreadInfo > , sleep : Sleep , injected_jobs : Injector < JobRef > , broadcasts : Mutex < Vec < Worker < JobRef > > > , panic_handler : Option < Box < PanicHandler > > , start_handler : Option < Box < StartHandler > > , exit_handler : Option < Box < ExitHandler > > , terminate_count : AtomicUsize , }
    };
}

Registry!();