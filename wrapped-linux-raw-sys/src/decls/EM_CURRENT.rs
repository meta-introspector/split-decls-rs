macro_rules! EM_CURRENT {
    () => {
        # [cfg (target_arch = "riscv64")] pub const EM_CURRENT : u16 = 243 ;
    };
}

EM_CURRENT!()