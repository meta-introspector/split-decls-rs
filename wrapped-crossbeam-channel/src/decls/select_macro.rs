macro_rules! select_macro {
    () => {
        # [cfg (feature = "std")] mod select_macro ;
    };
}

select_macro!()