macro_rules! deps {
    () => {
        Breaks!();
    };
}

macro_rules! PrintFrame {
    () => {
        deps!();
        # [derive (Copy , Clone)] enum PrintFrame { Fits , Broken { indent : usize , breaks : Breaks } , }
    };
}

PrintFrame!()