macro_rules! Registration {
    () => {
        # [doc = " A registration of a thread with an index."] # [doc = ""] # [doc = " When dropped, unregisters the thread and frees the reserved index."] struct Registration { index : usize , thread_id : ThreadId , }
    };
}

Registration!()