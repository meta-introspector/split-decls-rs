macro_rules! StyledValue {
    () => {
        # [cfg (not (feature = "color"))] type StyledValue < T > = T ;
    };
}

StyledValue!();