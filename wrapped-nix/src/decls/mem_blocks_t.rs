macro_rules! mem_blocks_t {
    () => {
        # [cfg (not (all (target_arch = "x86_64" , target_pointer_width = "32")))] type mem_blocks_t = libc :: c_ulong ;
    };
}

mem_blocks_t!()