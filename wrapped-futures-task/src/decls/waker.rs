macro_rules! waker {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod waker ;
    };
}

waker!();