macro_rules! speculation_barrier {
    () => {
        # [doc = " Equivalent to `__asm__ __volatile__(\"sb\" ::: \"memory\")`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_SB` is implemented."] # [inline] # [target_feature (enable = "sb")] unsafe fn speculation_barrier () { unsafe { asm ! ("sb" , options (nostack)) ; } }
    };
}

speculation_barrier!();