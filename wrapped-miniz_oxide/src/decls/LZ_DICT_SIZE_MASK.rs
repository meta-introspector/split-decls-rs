macro_rules! LZ_DICT_SIZE_MASK {
    () => {
        # [doc = " Mask used when stepping through the hash chains."] pub (crate) const LZ_DICT_SIZE_MASK : usize = (LZ_DICT_SIZE as u32 - 1) as usize ;
    };
}

LZ_DICT_SIZE_MASK!()