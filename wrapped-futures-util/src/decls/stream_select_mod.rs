macro_rules! stream_select_mod {
    () => {
        # [cfg (feature = "std")] # [cfg (feature = "async-await-macro")] mod stream_select_mod ;
    };
}

stream_select_mod!();