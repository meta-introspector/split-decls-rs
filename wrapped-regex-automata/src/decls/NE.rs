macro_rules! deps {
    () => {
        BE!();
    };
}

macro_rules! NE {
    () => {
        deps!();
        # [cfg (target_endian = "big")] pub (crate) type NE = BE ;
    };
}

NE!();