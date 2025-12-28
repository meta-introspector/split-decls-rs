macro_rules! select_jec {
    () => {
        # [inline] fn select_jec (word : usize) -> usize { word >> JEC_SHIFT }
    };
}

select_jec!();