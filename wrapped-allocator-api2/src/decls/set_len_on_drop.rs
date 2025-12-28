macro_rules! set_len_on_drop {
    () => {
        # [cfg (not (no_global_oom_handling))] mod set_len_on_drop ;
    };
}

set_len_on_drop!()