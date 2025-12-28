macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! downard {
    () => {
        deps!();
        # [inline] fn downard (_ : & mut ExtendedFloat , _ : bool) { }
    };
}

downard!();