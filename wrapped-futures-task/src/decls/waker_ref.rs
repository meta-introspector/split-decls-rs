macro_rules! waker_ref {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod waker_ref ;
    };
}

waker_ref!()