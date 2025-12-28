macro_rules! macro_1 {
    () => {
        # [cfg (not (icu4x_custom_data))] include ! ("../data/mod.rs") ;
    };
}

macro_1!();