macro_rules! load_4i {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn load_4i (s : & [u8]) -> i64 { load_4u (s) as i64 }
    };
}

load_4i!()