macro_rules! macro_0 {
    () => {
        # [cfg (icu4x_custom_data)] include ! (concat ! (core :: env ! ("ICU4X_DATA_DIR") , "/mod.rs")) ;
    };
}

macro_0!()