macro_rules! deps {
    () => {
        HashBuffers!();
    };
}

macro_rules! DictOxide {
    () => {
        deps!();
        pub (crate) struct DictOxide { # [doc = " The maximum number of checks in the hash chain, for the initial,"] # [doc = " and the lazy match respectively."] pub max_probes : [u32 ; 2] , # [doc = " Buffer of input data."] # [doc = " Padded with 1 byte to simplify matching code in `compress_fast`."] pub b : HashBuffers , pub code_buf_dict_pos : usize , pub lookahead_size : usize , pub lookahead_pos : usize , pub size : usize , loop_len : u8 , }
    };
}

DictOxide!();