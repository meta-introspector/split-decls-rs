macro_rules! current_index {
    () => {
        # [doc = " Returns a `usize` that identifies the current thread."] # [doc = ""] # [doc = " Each thread is associated with an 'index'. While there are no particular guarantees, indices"] # [doc = " usually tend to be consecutive numbers between 0 and the number of running threads."] # [doc = ""] # [doc = " Since this function accesses TLS, `None` might be returned if the current thread's TLS is"] # [doc = " tearing down."] # [inline] fn current_index () -> Option < usize > { REGISTRATION . try_with (| reg | reg . index) . ok () }
    };
}

current_index!();