macro_rules! deps {
    () => {
        Breaks!();
    };
}

macro_rules! PrintFrame {
    () => {
        deps!();
        # [derive (Copy , Clone)] enum PrintFrame { Fits (Breaks) , Broken (usize , Breaks) , }
    };
}

PrintFrame!();