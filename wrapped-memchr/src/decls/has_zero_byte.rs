macro_rules! has_zero_byte {
    () => {
        # [doc = " Return `true` if `x` contains any zero byte."] # [doc = ""] # [doc = " That is, this routine treats `x` as a register of 8-bit lanes and returns"] # [doc = " true when any of those lanes is `0`."] # [doc = ""] # [doc = " From \"Matters Computational\" by J. Arndt."] # [inline (always)] fn has_zero_byte (x : usize) -> bool { const LO : usize = splat (0x01) ; const HI : usize = splat (0x80) ; (x . wrapping_sub (LO) & ! x & HI) != 0 }
    };
}

has_zero_byte!();