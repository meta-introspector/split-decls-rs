macro_rules! z_off64_t {
    () => {
        # [cfg (all (windows , target_env = "gnu"))] pub type z_off64_t = z_off_t ;
    };
}

z_off64_t!();