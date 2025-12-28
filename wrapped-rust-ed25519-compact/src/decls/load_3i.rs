macro_rules! load_3i {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn load_3i (s : & [u8]) -> i64 { load_3u (s) as i64 }
    };
}

load_3i!();