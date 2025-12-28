macro_rules! select_mod {
    () => {
        # [cfg (feature = "async-await-macro")] mod select_mod ;
    };
}

select_mod!();