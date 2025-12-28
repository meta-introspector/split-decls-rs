macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! current {
    () => {
        deps!();
        # [doc = " Returns a handle to the current thread."] pub fn current () -> Thread { rt :: execution (| execution | { let thread = execution . threads . local (& CURRENT_THREAD_KEY) ; if let Some (thread) = thread { thread . unwrap () . clone () } else { init_current (execution , None) } }) }
    };
}

current!();