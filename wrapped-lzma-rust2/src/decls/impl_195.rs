macro_rules! deps {
    () => {
        FastEncoderMode!();
        LzEncoder!();
        MfType!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl FastEncoderMode { pub (crate) const EXTRA_SIZE_BEFORE : u32 = 1 ; pub (crate) const EXTRA_SIZE_AFTER : u32 = MATCH_LEN_MAX as u32 - 1 ; pub (crate) fn get_memory_usage (dict_size : u32 , extra_size_before : u32 , mf : MfType) -> u32 { LzEncoder :: get_memory_usage (dict_size , extra_size_before . max (Self :: EXTRA_SIZE_BEFORE) , Self :: EXTRA_SIZE_AFTER , MATCH_LEN_MAX as u32 , mf ,) } }
    };
}

impl_195!()