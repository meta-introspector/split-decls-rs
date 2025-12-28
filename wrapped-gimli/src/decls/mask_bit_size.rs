macro_rules! mask_bit_size {
    () => {
        # [inline] fn mask_bit_size (addr_mask : u64) -> u32 { 64 - addr_mask . leading_zeros () }
    };
}

mask_bit_size!()