macro_rules! min_sigstack_size {
    () => {
        # [doc = " Not all OS support hardware where this is needed."] # [cfg (not (any (target_os = "linux" , target_os = "android")))] fn min_sigstack_size () -> usize { libc :: MINSIGSTKSZ }
    };
}

min_sigstack_size!();