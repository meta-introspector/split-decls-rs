macro_rules! elf {
    () => {
        # [cfg (feature = "elf")] pub mod elf ;
    };
}

elf!();