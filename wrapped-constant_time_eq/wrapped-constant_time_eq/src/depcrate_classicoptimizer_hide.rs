// Generated macro for optimizer_hide (function)
macro_rules! Depcrate_classicoptimizer_hide {
() => {
// Module: crate::classic
// Provides: {"optimizer_hide"}
// Dependencies: {}
# [cfg (any (not (any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "arm" , target_arch = "aarch64" , target_arch = "riscv32" , target_arch = "riscv64" ,)) , miri ,))] # [inline (never)] # [must_use] fn optimizer_hide (value : u8) -> u8 { core :: hint :: black_box (value) }
};
}
