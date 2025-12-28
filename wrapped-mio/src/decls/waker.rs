macro_rules! waker {
    () => {
        # [cfg (not (target_os = "wasi"))] mod waker ;
    };
}

waker!();