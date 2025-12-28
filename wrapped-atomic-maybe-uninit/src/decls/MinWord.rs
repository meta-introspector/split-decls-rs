macro_rules! MinWord {
    () => {
        # [cfg (not (target_pointer_width = "16"))] type MinWord = u32 ;
    };
}

MinWord!()