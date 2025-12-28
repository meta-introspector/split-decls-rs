macro_rules! Md5Core {
    () => {
        # [doc = " Core MD5 hasher state."] # [derive (Clone)] pub struct Md5Core { block_len : u64 , state : [u32 ; STATE_LEN] , }
    };
}

Md5Core!()