macro_rules! RegContext {
    () => {
        # [derive (Debug)] pub struct RegContext { # [doc = " Hold the registers while the task or scheduler is suspended"] pub (crate) regs : Registers , }
    };
}

RegContext!();