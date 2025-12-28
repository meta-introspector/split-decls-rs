macro_rules! rsr64_dit {
    () => {
        # [doc = " Equivalent to `__arm_rsr64(\"dit\")`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_DIT` is implemented."] # [inline] # [target_feature (enable = "dit")] unsafe fn rsr64_dit () -> u64 { let mut value ; unsafe { asm ! ("mrs {}, dit" , lateout (reg) value , options (nomem , preserves_flags , nostack)) ; } value }
    };
}

rsr64_dit!()