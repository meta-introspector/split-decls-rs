macro_rules! asm {
    () => {
        # [cfg_attr (all (unix , target_arch = "aarch64") , path = "aarch64_unix.rs")] # [cfg_attr (all (unix , target_arch = "arm") , path = "arm_unix.rs")] # [cfg_attr (all (unix , target_arch = "x86_64") , path = "x86_64_unix.rs")] # [cfg_attr (all (windows , target_arch = "x86_64") , path = "x86_64_windows.rs")] # [cfg_attr (all (windows , target_arch = "aarch64") , path = "aarch64_windows.rs")] # [cfg_attr (all (unix , target_arch = "loongarch64") , path = "loongarch64_unix.rs")] # [cfg_attr (all (unix , target_arch = "riscv64") , path = "riscv64_unix.rs")] # [cfg_attr (all (unix , target_arch = "powerpc64") , path = "ppc64le_unix.rs")] pub mod asm ;
    };
}

asm!()