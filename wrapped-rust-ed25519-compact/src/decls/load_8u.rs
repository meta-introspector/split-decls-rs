macro_rules! load_8u {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] fn load_8u (s : & [u8]) -> u64 { (s [0] as u64) | ((s [1] as u64) << 8) | ((s [2] as u64) << 16) | ((s [3] as u64) << 24) | ((s [4] as u64) << 32) | ((s [5] as u64) << 40) | ((s [6] as u64) << 48) | ((s [7] as u64) << 56) }
    };
}

load_8u!()