macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! LockState {
    () => {
        deps!();
        trait LockState { fn is_locked (self) -> bool ; fn is_queue_locked (self) -> bool ; fn queue_head (self) -> * const ThreadData ; fn with_queue_head (self , thread_data : * const ThreadData) -> Self ; }
    };
}

LockState!()