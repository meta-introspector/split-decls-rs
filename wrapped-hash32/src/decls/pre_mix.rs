macro_rules! pre_mix {
    () => {
        fn pre_mix (mut block : u32) -> u32 { block = block . wrapping_mul (C1) ; block = block . rotate_left (R1) ; block = block . wrapping_mul (C2) ; block }
    };
}

pre_mix!()