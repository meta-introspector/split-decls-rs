macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Drop for FixedBitSet { fn drop (& mut self) { drop (unsafe { Vec :: from_raw_parts (self . data . as_ptr () , self . simd_block_len () , self . capacity) }) ; } }
    };
}

impl_88!();