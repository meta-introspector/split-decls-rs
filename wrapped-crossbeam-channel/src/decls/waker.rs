macro_rules! waker {
    () => {
        # [cfg (feature = "std")] mod waker ;
    };
}

waker!();