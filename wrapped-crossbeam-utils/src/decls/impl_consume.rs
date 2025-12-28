macro_rules! impl_consume {
    () => {
        # [cfg (not (crossbeam_no_atomic))] # [cfg (not (all (any (target_arch = "arm" , target_arch = "aarch64") , not (any (miri , crossbeam_loom , crossbeam_sanitize_thread)) ,)))] macro_rules ! impl_consume { () => { # [inline] fn load_consume (& self) -> Self :: Val { self . load (Ordering :: Acquire) } } ; }
    };
}

impl_consume!()