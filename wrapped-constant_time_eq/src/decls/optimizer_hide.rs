macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! optimizer_hide {
    () => {
        deps!();
        # [doc = " Attempts to hide a value from the optimizer."] # [cfg (any (miri , not (any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "arm" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "loongarch64" , target_arch = "s390x" ,))))] # [must_use] # [inline (never)] fn optimizer_hide (value : Word) -> Word { core :: hint :: black_box (value) }
    };
}

optimizer_hide!()