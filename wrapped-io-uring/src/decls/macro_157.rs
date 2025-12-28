macro_rules! macro_157 {
    () => {
        # [cfg (all (not (feature = "bindgen") , not (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "riscv64" , target_arch = "loongarch64" , target_arch = "powerpc64")) , not (io_uring_skip_arch_check)))] compile_error ! ("The prebuilt `sys.rs` may not be compatible with your target,
please use bindgen feature to generate new `sys.rs` of your arch
or use `--cfg=io_uring_skip_arch_check` to skip the check.") ;
    };
}

macro_157!()