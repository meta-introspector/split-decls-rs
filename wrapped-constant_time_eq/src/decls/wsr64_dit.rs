macro_rules! wsr64_dit {
    () => {
        # [doc = " Equivalent to `__arm_wsr64(\"dit\", value)`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_DIT` is implemented."] # [inline] # [target_feature (enable = "dit")] unsafe fn wsr64_dit (value : u64) { unsafe { asm ! ("msr dit, {}" , in (reg) value , options (nostack)) ; } }
    };
}

wsr64_dit!()