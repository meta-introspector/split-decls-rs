macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Executor {
    () => {
        deps!();
        # [doc = " The blocking executor."] struct Executor { # [doc = " Inner state of the executor."] inner : Mutex < Inner > , # [doc = " Used to put idle threads to sleep and wake them up when new work comes in."] cvar : Condvar , }
    };
}

Executor!();