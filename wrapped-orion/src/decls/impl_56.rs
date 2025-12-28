macro_rules! deps {
    () => {
        U64x4!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (test)] impl PartialEq < U64x4 > for U64x4 { fn eq (& self , other : & Self) -> bool { self . 0 == other . 0 && self . 1 == other . 1 && self . 2 == other . 2 && self . 3 == other . 3 } }
    };
}

impl_56!();