macro_rules! atomic_waker {
    () => {
        # [cfg_attr (target_os = "none" , cfg (any (target_has_atomic = "ptr" , feature = "portable-atomic")))] mod atomic_waker ;
    };
}

atomic_waker!();