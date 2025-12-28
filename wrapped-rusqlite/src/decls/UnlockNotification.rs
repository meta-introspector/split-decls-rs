macro_rules! UnlockNotification {
    () => {
        struct UnlockNotification { cond : Condvar , mutex : Mutex < bool > , }
    };
}

UnlockNotification!()