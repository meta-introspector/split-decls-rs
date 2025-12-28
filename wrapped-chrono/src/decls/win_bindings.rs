macro_rules! win_bindings {
    () => {
        # [cfg (all (windows , feature = "clock"))] # [allow (unreachable_pub)] mod win_bindings ;
    };
}

win_bindings!();