macro_rules! shr_odd_rounding {
    () => {
        # [doc = " Shift right, rounding all inexact divisions to the nearest odd number"] # [doc = " E.g. (0 >> 4) -> 0, (1..=31 >> 4) -> 1, (32 >> 4) -> 2, ..."] # [doc = ""] # [doc = " Useful for reducing a number before rounding the last two bits, since"] # [doc = " the result of the final rounding is preserved for all rounding modes."] const fn shr_odd_rounding (x : u128 , k : u32) -> u128 { if k < 128 { let inexact = x . trailing_zeros () < k ; (x >> k) | (inexact as u128) } else { (x != 0) as u128 } }
    };
}

shr_odd_rounding!();