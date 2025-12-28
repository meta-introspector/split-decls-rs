macro_rules! enable_dit {
    () => {
        # [doc = " Equivalent to `__arm_wsr64(\"dit\", 1 << 24)`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_DIT` is implemented."] # [inline] # [target_feature (enable = "dit")] unsafe fn enable_dit () { unsafe { asm ! ("msr dit, #{}" , const 1 , options (nostack)) ; } }
    };
}

enable_dit!();