macro_rules! lower_mult_add {
    () => {
        const fn lower_mult_add (x : u64 , y : u64) -> u64 { let mask = 0xFFFF_FFFFu64 ; let x_l = x & mask ; let y_l = y & mask ; let xy = x_l . wrapping_mul (y_l) ; x . wrapping_add (y . wrapping_add (xy . wrapping_add (xy))) }
    };
}

lower_mult_add!()