macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! impl_1073 {
    () => {
        deps!();
        impl Direction { # [doc = " Return the opposite `Direction`."] # [inline] pub fn opposite (self) -> Direction { match self { Outgoing => Incoming , Incoming => Outgoing , } } # [doc = " Return `0` for `Outgoing` and `1` for `Incoming`."] # [inline] pub fn index (self) -> usize { (self as usize) & 0x1 } }
    };
}

impl_1073!();