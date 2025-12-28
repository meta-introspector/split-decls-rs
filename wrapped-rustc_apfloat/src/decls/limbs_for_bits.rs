macro_rules! limbs_for_bits {
    () => {
        # [inline (always)] fn limbs_for_bits (bits : usize) -> usize { (bits + LIMB_BITS - 1) / LIMB_BITS }
    };
}

limbs_for_bits!();