macro_rules! timespec_tv_nsec_t {
    () => {
        # [cfg (not (all (target_arch = "x86_64" , target_pointer_width = "32")))] type timespec_tv_nsec_t = libc :: c_long ;
    };
}

timespec_tv_nsec_t!();