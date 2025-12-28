macro_rules! ReadyState {
    () => {
        struct ReadyState < T : Async > { set_completed : AtomicBool , result : Result < T :: Output > , }
    };
}

ReadyState!();